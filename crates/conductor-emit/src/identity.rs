//! Per-execution span identity: a content-preserving re-key of a built trace export.
//!
//! Every span builder draws `(trace_id, span_id)` from its seed, so two drives of one scenario at one
//! seed carry the same identities — and a span store keyed on that pair refuses the second drive while
//! it still holds the first. [`rekey_trace_identity`] moves identity and nothing else, so the stream's
//! content stays a function of scenario + seed while its identity also takes a per-execution salt.

use opentelemetry_proto::tonic::collector::trace::v1::ExportTraceServiceRequest;
use rand_chacha::ChaCha8Rng;
use rand_core::SeedableRng;

use crate::span_tree::gen_id;

/// Re-key every span identity in `request` under `salt`.
///
/// One `ChaCha8Rng::seed_from_u64(salt)` draws a 16-byte trace mask, THEN an 8-byte span mask (the
/// order is part of the contract). Each non-empty `trace_id` is XORed with the trace mask and each
/// non-empty `span_id` / `parent_span_id` — and every link's `trace_id` / `span_id` — with the span
/// mask. An EMPTY id stays empty, so a root keeps its empty parent. Consequences:
/// - a bijection per salt (XOR with a fixed mask is its own inverse);
/// - linkage preserved: a child's re-keyed `parent_span_id` equals its parent's re-keyed `span_id`;
/// - content untouched: no field other than those ids is read or written;
/// - a pure function of `(request, salt)` — no clock, no I/O, no self-observation.
///
/// A non-empty id can re-key to all zeroes only with probability 2^-64 (span) / 2^-128 (trace) per id;
/// that is not guarded.
pub fn rekey_trace_identity(request: &mut ExportTraceServiceRequest, salt: u64) {
    let mut rng = ChaCha8Rng::seed_from_u64(salt);
    let trace_mask = gen_id::<16>(&mut rng);
    let span_mask = gen_id::<8>(&mut rng);
    let spans = request
        .resource_spans
        .iter_mut()
        .flat_map(|r| r.scope_spans.iter_mut())
        .flat_map(|s| s.spans.iter_mut());
    for span in spans {
        xor_in_place(&mut span.trace_id, &trace_mask);
        xor_in_place(&mut span.span_id, &span_mask);
        xor_in_place(&mut span.parent_span_id, &span_mask);
        for link in &mut span.links {
            xor_in_place(&mut link.trace_id, &trace_mask);
            xor_in_place(&mut link.span_id, &span_mask);
        }
    }
}

fn xor_in_place(id: &mut [u8], mask: &[u8]) {
    for (byte, m) in id.iter_mut().zip(mask.iter().cycle()) {
        *byte ^= m;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        ErrorPlacement, ExceptionSpec, Frame, error_trace_request, exception_trace_request,
    };
    use opentelemetry_proto::tonic::trace::v1::Span;
    use std::collections::BTreeSet;

    fn spans_of(req: &ExportTraceServiceRequest) -> &[Span] {
        &req.resource_spans[0].scope_spans[0].spans
    }

    fn identities(req: &ExportTraceServiceRequest) -> BTreeSet<(Vec<u8>, Vec<u8>)> {
        spans_of(req)
            .iter()
            .map(|s| (s.trace_id.clone(), s.span_id.clone()))
            .collect()
    }

    fn deep_request() -> ExportTraceServiceRequest {
        error_trace_request("svc", 7, ErrorPlacement::DeepChild { depth: 3 }, "deep")
    }

    fn blank_ids(req: &mut ExportTraceServiceRequest) {
        for span in req
            .resource_spans
            .iter_mut()
            .flat_map(|r| r.scope_spans.iter_mut())
            .flat_map(|s| s.spans.iter_mut())
        {
            span.trace_id.clear();
            span.span_id.clear();
            span.parent_span_id.clear();
            for link in &mut span.links {
                link.trace_id.clear();
                link.span_id.clear();
            }
        }
    }

    #[test]
    fn rekey_preserves_parent_linkage_and_one_shared_trace_id() {
        let original = deep_request();
        let mut rekeyed = original.clone();
        rekey_trace_identity(&mut rekeyed, 0xC0FFEE);
        let spans = spans_of(&rekeyed);
        assert_eq!(spans.len(), 4);
        assert!(spans.iter().all(|s| s.trace_id == spans[0].trace_id));
        assert_ne!(spans[0].trace_id, spans_of(&original)[0].trace_id);
        assert!(
            spans
                .windows(2)
                .all(|w| w[1].parent_span_id == w[0].span_id)
        );
    }

    #[test]
    fn same_request_and_salt_rekey_identically() {
        let mut a = deep_request();
        let mut b = deep_request();
        rekey_trace_identity(&mut a, 42);
        rekey_trace_identity(&mut b, 42);
        assert_eq!(identities(&a), identities(&b));
        assert_eq!(
            spans_of(&a)
                .iter()
                .map(|s| &s.parent_span_id)
                .collect::<Vec<_>>(),
            spans_of(&b)
                .iter()
                .map(|s| &s.parent_span_id)
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn distinct_salts_share_no_identity() {
        let mut a = deep_request();
        let mut b = deep_request();
        rekey_trace_identity(&mut a, 1);
        rekey_trace_identity(&mut b, 2);
        let (ia, ib) = (identities(&a), identities(&b));
        assert_eq!(ia.len(), 4);
        assert!(ia.is_disjoint(&ib));
    }

    #[test]
    fn rekey_moves_no_content_byte() {
        let spec = ExceptionSpec::new(
            "IoError",
            "disk full",
            vec![Frame::new("write_block", "src/store.rs", 42)],
        );
        let original = exception_trace_request("svc", 4317006, &spec);
        let mut rekeyed = original.clone();
        rekey_trace_identity(&mut rekeyed, 99);
        assert_ne!(identities(&original), identities(&rekeyed));
        let mut original = original;
        blank_ids(&mut original);
        blank_ids(&mut rekeyed);
        assert_eq!(original, rekeyed);
    }

    #[test]
    fn rekey_under_one_salt_twice_restores_every_id() {
        let original = deep_request();
        let mut twice = original.clone();
        rekey_trace_identity(&mut twice, 0xC0FFEE);
        assert_ne!(twice, original);
        rekey_trace_identity(&mut twice, 0xC0FFEE);
        assert_eq!(twice, original);
    }

    #[test]
    fn empty_parent_span_id_stays_empty() {
        let mut req = deep_request();
        rekey_trace_identity(&mut req, 7);
        let spans = spans_of(&req);
        assert!(spans[0].parent_span_id.is_empty());
        assert!(spans[1..].iter().all(|s| s.parent_span_id.len() == 8));
    }
}
