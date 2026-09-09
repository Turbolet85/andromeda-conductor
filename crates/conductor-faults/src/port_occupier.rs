//! Port-occupier fault (P-003): a sacrificial listener that holds Pulse's OTLP/gRPC ingest port so
//! Pulse's own receiver cannot bind it, driving the `ReceiverFailed` connection state.
//!
//! [`PortOccupier`] binds `127.0.0.1:<port>` — loopback hard-coded, never a routable interface — and
//! holds it until dropped or [`PortOccupier::release`]d. It never `accept`s a connection and never
//! speaks OTLP; binding alone denies the port. This is Conductor's *only* deliberate inbound bind
//! (architecture §Occupied Resources / §Cross-cutting Patterns — Trust boundary). A refused bind is a
//! typed [`FaultError`], never a panic.

use std::net::{Ipv4Addr, SocketAddr, TcpListener};
use std::time::Instant;

use crate::error::FaultError;

/// Pulse's OTLP/gRPC ingest port — the address the occupier denies by default.
pub const OTLP_INGEST_PORT: u16 = 4317;

/// A sacrificial hold on a loopback TCP port: holds the bound socket until released or dropped.
/// `Drop` releases it (RAII) so a subsequent scenario's egress reaches the real Pulse.
#[derive(Debug)]
pub struct PortOccupier {
    addr: SocketAddr,
    listener: Option<TcpListener>,
    /// The `fault.port_occupier` span, open for exactly as long as the bind is held (obs-plan §4).
    span: Option<tracing::Span>,
    held_since: Instant,
}

impl PortOccupier {
    /// Bind and hold `127.0.0.1:port`. The IP is fixed to loopback — only the port is chosen, so the
    /// occupier can never open a routable interface. Pass `0` to take an OS-assigned ephemeral port
    /// (tests do this). A refused bind (the port is already held) is a typed [`FaultError`], not a panic.
    pub fn occupy(port: u16) -> Result<Self, FaultError> {
        let requested = SocketAddr::from((Ipv4Addr::LOCALHOST, port));
        let listener = TcpListener::bind(requested).map_err(|source| FaultError::Bind {
            addr: requested,
            source,
        })?;
        let addr = listener.local_addr().map_err(|source| FaultError::Bind {
            addr: requested,
            source,
        })?;
        // Created, never entered — the span brackets the hold in the self-obs stream; only the two
        // values knowable at bind time ride it (obs-plan §4; the layer records attributes on `new`).
        let span = tracing::info_span!(
            "fault.port_occupier",
            fault_type = "port_occupier",
            port = addr.port()
        );
        Ok(Self {
            addr,
            listener: Some(listener),
            span: Some(span),
            held_since: Instant::now(),
        })
    }

    /// Bind and hold the default OTLP ingest port ([`OTLP_INGEST_PORT`]).
    pub fn occupy_default() -> Result<Self, FaultError> {
        Self::occupy(OTLP_INGEST_PORT)
    }

    /// The bound loopback address — the resolved port (the real OS-assigned one when `0` was passed).
    pub fn local_addr(&self) -> SocketAddr {
        self.addr
    }

    /// Release the port by dropping the held socket. Idempotent — a second call is a no-op.
    ///
    /// The realized hold rides the allowlisted `message` field rather than a span attribute: it is
    /// knowable only here, and the subscriber records span attributes on `new` alone (obs-plan §6).
    pub fn release(&mut self) {
        self.listener = None;
        if let Some(span) = self.span.take() {
            span.in_scope(|| {
                tracing::debug!(
                    "port occupier released after {}ms",
                    self.held_since.elapsed().as_millis()
                );
            });
        }
    }
}

impl Drop for PortOccupier {
    fn drop(&mut self) {
        self.release();
    }
}
