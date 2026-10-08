
## 2026-10-08-version-close-on-measured-evidence — the posture contract's entry says what reads it; the `contracts/` tree line follows
**Section:** §Occupied Resources → On-disk artifacts, the `contracts/pulse-real-model-leg-posture.md` entry · §Infrastructure Patterns → Directory structure (crate-per-seam Cargo workspace), the `contracts/` line
**Change:**
- The entry said "It is the **SECOND** `contracts/` member with **NO Rust reader**, taking the P-025 regime above unchanged."; it now says "… with **NO Rust reader** but a test-tier digest hold (its `## Regime`), taking the P-025 regime above unchanged." The ordinal "SECOND" and the P-025 entry's "FIRST" stand as written.
- To pay for it, the entry's face-field sentence went to by-reference form: was "Carries `sut_version` · `captured_at` · `pinned_at` · `provenance` on its own face."; now "Carries the same four face fields." The four names stand in full in the P-025 entry directly above.
- The tree's `contracts/` line said "(the two members no Rust code reads)"; now "(the two members no shipped code reads)".
- §Occupied Resources measured 38114 B before, 38062 B after the freeing step and 38108 B after the correction, against the 38115 B threshold; §Established Decisions 38082 B, unchanged. Both within target.
**Why:** since the 2026-09-30 series a `conductor-run` test helper holds each dated section of the contract by sha256 and parses nothing, while no shipped code reads the file, so "no Rust reader" was false of this member as written. The founder ruled by dialog on 2026-10-08, relayed by the pc overseer, that a test-tier digest read is not the runtime read the contract's notice names, so the regime clause stands. The operator overruled paying for the correction with an ordinal: a historical ordinal stays as written. For a later chunk: the section has 7 B of headroom, so any growth frees bytes first.
**Kept:** the P-025 entry is unchanged; it has no Rust reader by name. The kind and tier of the reader live in the contract's dated `## Regime` block and in security-plan's fixed-path manifests row, which the entry points at.
**Ref:** .andromeda/runs/2026-10-08T10-43-15-wrap/
