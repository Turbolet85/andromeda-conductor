//! Multi-service topology traces: one OTLP export carrying ≥2 distinct `service.name` `ResourceSpans`
//! (the constellation Pulse renders as service dots — P-027) joined into one trace by a shared seeded
//! `trace_id` and cross-service `parent_span_id` linkage (W3C Trace Context across emulated service
//! edges). An optional [`ErrorPlacement`] sets `Status.Code=ERROR` on exactly one service's span —
//! the cross-service face of root-vs-deep error placement (P-008). Identity is a deterministic
//! function of the seed (architecture §Established Decisions — Determinism RNG): the builder reads no
//! clock for identity and owns no global entropy, reusing the shared raw-struct primitives
//! ([`crate::message`]) and the seeded id generator ([`crate::span_tree::gen_id`]).

use opentelemetry_proto::tonic::collector::trace::v1::ExportTraceServiceRequest;
use opentelemetry_proto::tonic::trace::v1::{ResourceSpans, ScopeSpans};
use rand_chacha::ChaCha8Rng;
use rand_core::SeedableRng;

use crate::message::{error_status, ok_status, service_resource, span};
use crate::span_tree::{gen_id, ErrorPlacement};

/// An ordered, root-first chain of distinct service names. Constructed only with ≥2 distinct
/// services, so an emitted topology always realizes a genuine multi-service constellation (P-027) —
/// mirroring the self-validating constructors of [`crate::latency::LatencyProfile`] /
/// [`crate::logs::Severity`]; the `conductor-core` garde layer is the authoritative validator once
/// topologies join the scenario model.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServiceTopology {
    services: Vec<String>,
}

impl ServiceTopology {
    /// Build a topology; `None` unless `services` holds ≥2 entries that are all distinct.
    pub fn new(services: &[&str]) -> Option<Self> {
        if services.len() < 2 {
            return None;
        }
        let all_distinct = services
            .iter()
            .enumerate()
            .all(|(i, name)| !services[i + 1..].contains(name));
        all_distinct.then(|| Self {
            services: services.iter().map(|s| s.to_string()).collect(),
        })
    }

    /// The ordered, root-first service names (length ≥ 2).
    pub fn services(&self) -> &[String] {
        &self.services
    }
}

/// Build an OTLP trace export spanning [`ServiceTopology`]'s services: one `ResourceSpans` per
/// service (each with its own `service.name`), one span per service, all sharing one seeded
/// `trace_id`; the root span has no parent and each downstream span links to the upstream service's
/// `span_id` (W3C cross-service propagation). `placement` selects which service's span carries
/// `Status.Code=ERROR` ([`ErrorPlacement::Root`] = the root service; [`ErrorPlacement::DeepChild`]
/// `{ depth }` = service `depth`, saturated to the deepest service); `None` emits an all-`OK` trace.
/// Identity is a deterministic function of `seed`; `start`/`end` stamps are wall-clock.
pub fn service_topology_request(
    topology: &ServiceTopology,
    seed: u64,
    placement: Option<ErrorPlacement>,
    message: &str,
) -> ExportTraceServiceRequest {
    let services = topology.services();
    let deepest = services.len() - 1;
    let error_index = placement.map(|p| match p {
        ErrorPlacement::Root => 0,
        ErrorPlacement::DeepChild { depth } => depth.min(deepest),
    });

    let mut rng = ChaCha8Rng::seed_from_u64(seed);
    let trace_id = gen_id::<16>(&mut rng);

    let mut span_ids: Vec<Vec<u8>> = Vec::with_capacity(services.len());
    let mut resource_spans: Vec<ResourceSpans> = Vec::with_capacity(services.len());
    for (index, name) in services.iter().enumerate() {
        let span_id = gen_id::<8>(&mut rng).to_vec();
        let parent_span_id = if index == 0 {
            Vec::new()
        } else {
            span_ids[index - 1].clone()
        };
        let status = if Some(index) == error_index {
            error_status(message)
        } else {
            ok_status()
        };
        let name_label = if index == 0 {
            "root".to_string()
        } else {
            format!("child-{index}")
        };
        let built = span(
            &name_label,
            trace_id.to_vec(),
            span_id.clone(),
            parent_span_id,
            status,
        );
        span_ids.push(span_id);
        resource_spans.push(ResourceSpans {
            resource: Some(service_resource(name)),
            scope_spans: vec![ScopeSpans {
                spans: vec![built],
                ..Default::default()
            }],
            ..Default::default()
        });
    }

    ExportTraceServiceRequest { resource_spans }
}

#[cfg(test)]
mod tests {
    use super::*;
    use opentelemetry_proto::tonic::common::v1::any_value;
    use opentelemetry_proto::tonic::trace::v1::{status::StatusCode, Span};

    fn three() -> [&'static str; 3] {
        ["gateway", "api", "db"]
    }

    /// The one span per service, in topology order.
    fn spans_in_order(req: &ExportTraceServiceRequest) -> Vec<&Span> {
        req.resource_spans
            .iter()
            .map(|rs| &rs.scope_spans[0].spans[0])
            .collect()
    }

    fn service_names(req: &ExportTraceServiceRequest) -> Vec<String> {
        req.resource_spans
            .iter()
            .map(|rs| {
                let kv = rs
                    .resource
                    .as_ref()
                    .unwrap()
                    .attributes
                    .iter()
                    .find(|kv| kv.key == "service.name")
                    .unwrap();
                match kv.value.as_ref().unwrap().value.as_ref().unwrap() {
                    any_value::Value::StringValue(s) => s.clone(),
                    _ => panic!("service.name not a string"),
                }
            })
            .collect()
    }

    /// The seeded shape (identity + linkage + status), excluding wall-clock — the determinism contract.
    type SpanShape = (Vec<u8>, Vec<u8>, Vec<u8>, i32);
    fn shape(req: &ExportTraceServiceRequest) -> Vec<SpanShape> {
        spans_in_order(req)
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
    fn rejects_fewer_than_two_services() {
        assert!(ServiceTopology::new(&["only"]).is_none());
        assert!(ServiceTopology::new(&[]).is_none());
    }

    #[test]
    fn rejects_duplicate_service_names() {
        assert!(ServiceTopology::new(&["api", "db", "api"]).is_none());
    }

    #[test]
    fn accepts_two_or_more_distinct() {
        let t = ServiceTopology::new(&["a", "b"]).unwrap();
        assert_eq!(t.services(), &["a", "b"]);
    }

    #[test]
    fn emits_one_resource_spans_per_distinct_service() {
        let t = ServiceTopology::new(&three()).unwrap();
        let req = service_topology_request(&t, 7, None, "");
        assert_eq!(req.resource_spans.len(), 3);
        assert_eq!(service_names(&req), vec!["gateway", "api", "db"]);
        assert!(req
            .resource_spans
            .iter()
            .all(|rs| rs.scope_spans[0].spans.len() == 1));
    }

    #[test]
    fn shares_one_trace_id_with_cross_service_parent_linkage() {
        let t = ServiceTopology::new(&three()).unwrap();
        let req = service_topology_request(&t, 7, None, "");
        let spans = spans_in_order(&req);
        let trace_id = &spans[0].trace_id;
        assert_eq!(trace_id.len(), 16);
        assert!(spans.iter().all(|s| &s.trace_id == trace_id));
        assert!(spans[0].parent_span_id.is_empty());
        assert!(spans.windows(2).all(|w| w[1].parent_span_id == w[0].span_id));
        let mut ids: Vec<&Vec<u8>> = spans.iter().map(|s| &s.span_id).collect();
        ids.sort();
        ids.dedup();
        assert_eq!(ids.len(), spans.len());
    }

    #[test]
    fn healthy_topology_is_all_ok() {
        let t = ServiceTopology::new(&three()).unwrap();
        let req = service_topology_request(&t, 7, None, "");
        assert!(spans_in_order(&req)
            .iter()
            .all(|s| s.status.as_ref().unwrap().code == StatusCode::Ok as i32));
    }

    #[test]
    fn root_placement_errors_the_root_service_only() {
        let t = ServiceTopology::new(&three()).unwrap();
        let req = service_topology_request(&t, 7, Some(ErrorPlacement::Root), "boom");
        let spans = spans_in_order(&req);
        let root = spans[0].status.as_ref().unwrap();
        assert_eq!(root.code, StatusCode::Error as i32);
        assert_eq!(root.message, "boom");
        assert!(spans[1..]
            .iter()
            .all(|s| s.status.as_ref().unwrap().code == StatusCode::Ok as i32));
    }

    #[test]
    fn deep_placement_errors_the_downstream_service_only() {
        let t = ServiceTopology::new(&three()).unwrap();
        let req = service_topology_request(&t, 7, Some(ErrorPlacement::DeepChild { depth: 2 }), "deep");
        let spans = spans_in_order(&req);
        let leaf = spans[2].status.as_ref().unwrap();
        assert_eq!(leaf.code, StatusCode::Error as i32);
        assert_eq!(leaf.message, "deep");
        assert!(spans[..2]
            .iter()
            .all(|s| s.status.as_ref().unwrap().code == StatusCode::Ok as i32));
    }

    #[test]
    fn deep_placement_saturates_to_the_deepest_service() {
        let t = ServiceTopology::new(&three()).unwrap();
        let req = service_topology_request(&t, 7, Some(ErrorPlacement::DeepChild { depth: 99 }), "x");
        let spans = spans_in_order(&req);
        assert_eq!(spans[2].status.as_ref().unwrap().code, StatusCode::Error as i32);
        assert!(spans[..2]
            .iter()
            .all(|s| s.status.as_ref().unwrap().code == StatusCode::Ok as i32));
    }

    #[test]
    fn same_seed_reproduces_identical_shape() {
        let t = ServiceTopology::new(&three()).unwrap();
        let a = service_topology_request(&t, 123, Some(ErrorPlacement::DeepChild { depth: 1 }), "x");
        let b = service_topology_request(&t, 123, Some(ErrorPlacement::DeepChild { depth: 1 }), "x");
        assert_eq!(shape(&a), shape(&b));
    }

    #[test]
    fn different_seeds_diverge() {
        let t = ServiceTopology::new(&three()).unwrap();
        let a = service_topology_request(&t, 1, None, "");
        let b = service_topology_request(&t, 2, None, "");
        assert_ne!(spans_in_order(&a)[0].trace_id, spans_in_order(&b)[0].trace_id);
        assert_ne!(shape(&a), shape(&b));
    }
}
