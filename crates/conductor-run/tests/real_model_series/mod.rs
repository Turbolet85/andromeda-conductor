//! The 2026-09-29 real-model drive series, pinned: each drive's committed `evidence/rm-capture-*.txt`
//! held by the sha256 digest of its LF-normalized content. No capture text sits here — the harvest
//! reads each committed file, checks its digest, and grades the block the capture printed after its
//! rule record (the 2026-09-23 pin's cut). A `tests/` subdirectory module, so it is never a test
//! target of its own.

/// One drive of the series: its ledger label, its committed capture's file name, and the sha256 (lower
/// hex) of that file's LF-normalized content.
pub struct Drive {
    pub label: &'static str,
    pub file: &'static str,
    pub sha256: &'static str,
}

/// The committed captures' directory, relative to the workspace root.
pub const EVIDENCE: &str =
    "conductor-0.3.0/chunks/2026-09-29-diagnostic-quality-cluster-off-the-drift-pin/evidence";

pub const SERIES: [Drive; 6] = [
    Drive {
        label: "a1-pipeline-fault",
        file: "rm-capture-a1-pipeline-fault.txt",
        sha256: "63139da795c609929dd262d3dba815ffd735dc278e41cd38af115cfc43e2d5a5",
    },
    Drive {
        label: "a1",
        file: "rm-capture-a1.txt",
        sha256: "a0cb122b6962c680d54d8a191a173d3626b0673631d9556fa9cb97972c239015",
    },
    Drive {
        label: "a2",
        file: "rm-capture-a2.txt",
        sha256: "c9c58b2328636b94abbe3c02dcd718a31fc36d4840724bfaed856bdaf37e3e74",
    },
    Drive {
        label: "a3",
        file: "rm-capture-a3.txt",
        sha256: "5f5dfb00d5b57b391b813d8644f2dfb701305694453b085193b1e2a4a640e519",
    },
    Drive {
        label: "b1",
        file: "rm-capture-b1.txt",
        sha256: "ea7a347622b330292e484388c87904b3a04c7fc49b90f9eeb5428d25f9a670a3",
    },
    Drive {
        label: "b2",
        file: "rm-capture-b2.txt",
        sha256: "b01f04effff040017ebd7a8c822a6b3e4dd909c76886c86ac0ab7770f94393af",
    },
];

/// The 2026-09-30 series' committed captures (`contracts/pulse-real-model-leg-posture.md`, The 2026-09-30
/// series), relative to the workspace root.
pub const EVIDENCE_2026_09_30: &str =
    "conductor-0.3.0/chunks/2026-09-30-interpretation-re-proven-on-a-clean-named-data-dir/evidence";

/// The 2026-09-30 series: three drives on one fresh letters-only data dir, against andromeda-pulse `fcc31b2`.
pub const SERIES_2026_09_30: [Drive; 3] = [
    Drive {
        label: "d1",
        file: "rm-capture-d1.txt",
        sha256: "22118eb7f1c24ad48cb5f5d5b341a73ca2abe50eadf0839933c327353f4cc528",
    },
    Drive {
        label: "d2",
        file: "rm-capture-d2.txt",
        sha256: "f437b0760e8ac3b6475845a64246c1a6491232b78e4be10917a7f08ed672ea4a",
    },
    Drive {
        label: "d3",
        file: "rm-capture-d3.txt",
        sha256: "93b03a9e1dfa9c46fe05fb9421b31320c32ce852a6b870496b3c425212456208",
    },
];
