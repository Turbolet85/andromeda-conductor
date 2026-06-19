//! Harness-fault error type for the fault-injection seam — the verdict/error-wall half.
//!
//! A fault helper that cannot establish its condition (here, a port bind refused because the port is
//! already held — e.g. a live Pulse is up) fails in `Result::Err`, never a verification outcome — the
//! same wall `conductor_core::CoreError` draws for the core seam. `#[non_exhaustive]` so later Epoch-4
//! fault helpers extend the surface.

use std::io;
use std::net::SocketAddr;

/// A harness fault raised by a fault helper — never a verification verdict.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum FaultError {
    /// The port-occupier could not bind its loopback address (already held, or denied by the OS).
    #[error("could not bind port-occupier to {addr}")]
    Bind {
        addr: SocketAddr,
        #[source]
        source: io::Error,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bind_error_displays_its_target_addr_and_chains_the_io_source() {
        let addr = "127.0.0.1:4317".parse().unwrap();
        let err = FaultError::Bind {
            addr,
            source: io::Error::new(io::ErrorKind::AddrInUse, "address already in use"),
        };
        assert_eq!(err.to_string(), "could not bind port-occupier to 127.0.0.1:4317");
        // the underlying io::Error is preserved as the chained source (the verdict/error wall).
        assert!(std::error::Error::source(&err).is_some());
    }
}
