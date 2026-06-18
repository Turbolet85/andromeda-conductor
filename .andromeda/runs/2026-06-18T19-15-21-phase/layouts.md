# layouts extract

## Relevance
Out-of-scope for this chunk.

## No domain coverage
The PII payload corpus chunk (Epoch 3, emission primitives) is a backend `conductor-emit` crate module—seeded corpus generation, OTLP wire embedding, and unit/loopback tests. It touches no UI surfaces, layout structure, component placement, focus order, responsiveness, modal patterns, or navigation (per layout-templates §Primary Surfaces desktop-webview / cli, which are the only layout-bearing surfaces in Conductor). Layouts domain applies only to rendered surfaces; this chunk is pure emission-side backend infrastructure with no user-facing surface manifestation.