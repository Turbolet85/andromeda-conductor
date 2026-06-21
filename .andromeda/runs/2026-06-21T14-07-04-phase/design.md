# design extract

## No domain coverage

This chunk is a backend gRPC connectivity primitive with no rendered UI surface (a liveness probe over the OTLP egress channel to `:4317`, returning `Ok(()) | Result::Err` — never a verdict value). The design system applies only to the desktop-webview and cli surfaces; backend transport validation creates no design artifact.
