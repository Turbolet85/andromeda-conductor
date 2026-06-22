# layouts extract

## No domain coverage
Out-of-scope — chunk scope is error-baseline-spike + latency-regression scenario TOML fixture creation and round-trip validation (Epoch 7, artifact-only). Layouts covers surface structure (desktop-webview / cli wireframes, component placement, focus order) per layout-templates.md §Primary Surfaces. This chunk touches zero UI surfaces — declarative TOML scenario config + deserialize/validate proofs, no rendering change. The scenario timelines are consumed by Epoch-8's CLI driver (which is a layouts surface), but that chunk inherits the surfaces-already-live layout contract. No new layout template entry or surface modification here.
