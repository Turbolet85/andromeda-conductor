//! Multi-span error traces: a seeded root→child chain carrying one `Status.Code=ERROR` span,
//! placeable at the root or a deep child (P-005 hard error signal; P-008 root-vs-deep placement,
//! emission side). Span/trace identity derives from a seeded `ChaCha8Rng` taken as input, so the
//! same seed yields the same trace shape (architecture §Established Decisions — Determinism RNG);
//! the builder owns no global entropy and reads no clock for identity.

use opentelemetry_proto::tonic::collector::trace::v1::ExportTraceServiceRequest;
use opentelemetry_proto::tonic::trace::v1::{ResourceSpans, ScopeSpans, Span};
use rand_chacha::ChaCha8Rng;
use rand_core::{RngCore, SeedableRng};

use crate::message::{error_status, ok_status, service_resource, span};

/// Where the single `Status.Code=ERROR` span sits in the trace's root→child chain.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorPlacement {
    /// The error is on the root span (depth 0): a single-span trace.
    Root,
    /// The error is on a descendant `depth` levels below the root — a `depth + 1`-span chain whose
    /// leaf carries the error.
    DeepChild { depth: usize },
}

impl ErrorPlacement {
    fn depth(self) -> usize {
        match self {
            ErrorPlacement::Root => 0,
            ErrorPlacement::DeepChild { depth } => depth,
        }
    }
}

/// Build an OTLP trace export of a root→child span chain in which exactly the placement-selected
/// (deepest) span carries `Status.Code=ERROR` with `message`; every ancestor carries `OK`. All
/// spans share one seeded `trace_id`; the root's `parent_span_id` is empty and each child links to
/// its parent's `span_id`. Identity is a deterministic function of `seed`.
pub fn error_trace_request(
    service_name: &str,
    seed: u64,
    placement: ErrorPlacement,
    message: &str,
) -> ExportTraceServiceRequest {
    let mut rng = ChaCha8Rng::seed_from_u64(seed);
    let trace_id = gen_id::<16>(&mut rng);
    let error_index = placement.depth();

    let mut spans: Vec<Span> = Vec::with_capacity(error_index + 1);
    for index in 0..=error_index {
        let span_id = gen_id::<8>(&mut rng).to_vec();
        let parent_span_id = if index == 0 {
            Vec::new()
        } else {
            spans[index - 1].span_id.clone()
        };
        let status = if index == error_index {
            error_status(message)
        } else {
            ok_status()
        };
        let name = if index == 0 {
            "root".to_string()
        } else {
            format!("child-{index}")
        };
        spans.push(span(&name, trace_id.to_vec(), span_id, parent_span_id, status));
    }

    ExportTraceServiceRequest {
        resource_spans: vec![ResourceSpans {
            resource: Some(service_resource(service_name)),
            scope_spans: vec![ScopeSpans {
                spans,
                ..Default::default()
            }],
            ..Default::default()
        }],
    }
}

fn gen_id<const N: usize>(rng: &mut ChaCha8Rng) -> [u8; N] {
    let mut id = [0u8; N];
    rng.fill_bytes(&mut id);
    id
}

#[cfg(test)]
mod tests {
    use super::*;
    use opentelemetry_proto::tonic::trace::v1::status::StatusCode;

    fn spans_of(req: &ExportTraceServiceRequest) -> &[Span] {
        &req.resource_spans[0].scope_spans[0].spans
    }

    /// One span's seeded shape: `(trace_id, span_id, parent_span_id, status_code)`.
    type SpanShape = (Vec<u8>, Vec<u8>, Vec<u8>, i32);

    /// The seeded shape (identity + linkage + status), excluding wall-clock timestamps — the part
    /// the determinism contract governs.
    fn shape(req: &ExportTraceServiceRequest) -> Vec<SpanShape> {
        spans_of(req)
            .iter()
            .map(|s| {
                (
                    s.trace_id.clone(),
                    s.span_id.clone(),
                    s.parent_span_id.clone(),
                    s.status.as_ref().unwrap().code,
                )
            })
            .collect()
    }

    #[test]
    fn root_placement_is_a_single_error_span() {
        let req = error_trace_request("svc", 7, ErrorPlacement::Root, "boom");
        let spans = spans_of(&req);
        assert_eq!(spans.len(), 1);
        let root = &spans[0];
        assert!(root.parent_span_id.is_empty());
        assert_eq!(root.trace_id.len(), 16);
        assert_eq!(root.span_id.len(), 8);
        let status = root.status.as_ref().unwrap();
        assert_eq!(status.code, StatusCode::Error as i32);
        assert_eq!(status.message, "boom");
    }

    #[test]
    fn deep_child_places_error_at_the_leaf_with_a_well_formed_chain() {
        let depth = 3;
        let req = error_trace_request("svc", 7, ErrorPlacement::DeepChild { depth }, "deep");
        let spans = spans_of(&req);
        assert_eq!(spans.len(), depth + 1);

        // one shared trace_id; root has no parent
        let trace_id = &spans[0].trace_id;
        assert!(spans[0].parent_span_id.is_empty());
        assert!(spans.iter().all(|s| &s.trace_id == trace_id));
        // each child links to its parent's span_id
        assert!(spans.windows(2).all(|w| w[1].parent_span_id == w[0].span_id));
        // exactly the leaf is ERROR; every ancestor is OK
        let leaf = &spans[depth];
        assert_eq!(leaf.status.as_ref().unwrap().code, StatusCode::Error as i32);
        assert_eq!(leaf.status.as_ref().unwrap().message, "deep");
        assert!(spans[..depth]
            .iter()
            .all(|s| s.status.as_ref().unwrap().code == StatusCode::Ok as i32));
        // span_ids are pairwise distinct
        let mut ids: Vec<&Vec<u8>> = spans.iter().map(|s| &s.span_id).collect();
        ids.sort();
        ids.dedup();
        assert_eq!(ids.len(), spans.len());
    }

    #[test]
    fn same_seed_reproduces_identical_shape() {
        let a = error_trace_request("svc", 99, ErrorPlacement::DeepChild { depth: 2 }, "x");
        let b = error_trace_request("svc", 99, ErrorPlacement::DeepChild { depth: 2 }, "x");
        assert_eq!(shape(&a), shape(&b));
    }

    #[test]
    fn different_seeds_diverge() {
        // ≥2 fixed seeds over multiple draws (trace_id + 3 span_ids) — proves the seed is wired
        // through to the identity bytes, not merely that the builder is reproducible.
        let a = error_trace_request("svc", 1, ErrorPlacement::DeepChild { depth: 2 }, "x");
        let b = error_trace_request("svc", 2, ErrorPlacement::DeepChild { depth: 2 }, "x");
        assert_ne!(spans_of(&a)[0].trace_id, spans_of(&b)[0].trace_id);
        assert_ne!(shape(&a), shape(&b));
    }
}
