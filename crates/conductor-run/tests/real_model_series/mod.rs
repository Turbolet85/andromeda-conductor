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

/// The 2026-10-01 series' committed captures (`contracts/pulse-real-model-leg-posture.md`, The 2026-10-01
/// series), relative to the workspace root.
pub const EVIDENCE_2026_10_01: &str = "conductor-0.3.0/chunks/2026-10-01-interpretation-re-proven-after-the-incident-surfacing-fix/evidence";

/// The 2026-10-01 series: three drives on one fresh letters-only data dir, against andromeda-pulse `a2addb3`.
pub const SERIES_2026_10_01: [Drive; 3] = [
    Drive {
        label: "d1",
        file: "rm-capture-d1.txt",
        sha256: "c81812379cf2221a706bfb63367ead80f45bbe0bc01e407ab0cac4813a4234b0",
    },
    Drive {
        label: "d2",
        file: "rm-capture-d2.txt",
        sha256: "92937807c03394daf34be89d960957f0f789b185f58c1ba979d0d81b38af7414",
    },
    Drive {
        label: "d3",
        file: "rm-capture-d3.txt",
        sha256: "b49bfe68c0210a2b4c50edbf54257c63980a95a133fcd6641f28e23a2d630015",
    },
];

/// The 2026-10-06 series' committed captures (`contracts/pulse-real-model-leg-posture.md`, The 2026-10-06
/// series), relative to the workspace root.
pub const EVIDENCE_2026_10_06: &str = "conductor-0.3.0/chunks/2026-10-06-a-fourth-pre-registered-real-model-series-for-v3-09/evidence";

/// The 2026-10-06 series: three drives on one fresh letters-only data dir, against andromeda-pulse `5f77859`.
pub const SERIES_2026_10_06: [Drive; 3] = [
    Drive {
        label: "d1",
        file: "rm-capture-d1.txt",
        sha256: "27a1222e5f7cce189c8dbe72a74bbd41309d1fcfee4ceec9ad47b74638b9da69",
    },
    Drive {
        label: "d2",
        file: "rm-capture-d2.txt",
        sha256: "fb77ce86f21329c33b1a4fa4291c75682a14f05d9709f07acce8bb1b1490c192",
    },
    Drive {
        label: "d3",
        file: "rm-capture-d3.txt",
        sha256: "12fcffb39dfccbad58eec1215b569e8b13e901ed5c8963d784abe864d689a8ae",
    },
];

/// The 2026-10-07 series' committed captures (`contracts/pulse-real-model-leg-posture.md`, The 2026-10-07
/// series), relative to the workspace root.
pub const EVIDENCE_2026_10_07: &str =
    "conductor-0.3.0/chunks/2026-10-07-a-fifth-pre-registered-real-model-series-for-v3-09/evidence";

/// The 2026-10-07 series: three drives on one fresh letters-only data dir, against andromeda-pulse `f70be92`.
pub const SERIES_2026_10_07: [Drive; 3] = [
    Drive {
        label: "d1",
        file: "rm-capture-d1.txt",
        sha256: "1f434d2b818f6c2bb0ce184ef83af7bc9d309a951e93f32515fc224f6d0f8770",
    },
    Drive {
        label: "d2",
        file: "rm-capture-d2.txt",
        sha256: "f5a2cc300e1f9236d6f5e0f9bda6fc9de561b28685a9c9a508eb661330ce791d",
    },
    Drive {
        label: "d3",
        file: "rm-capture-d3.txt",
        sha256: "cdc403cbcab2b2e92bbd28e5e14fda9880c42406b492e151e18a259071c33815",
    },
];

/// The 2026-10-07 capture run's committed captures (`contracts/pulse-real-model-leg-posture.md`, The
/// 2026-10-07 capture run), relative to the workspace root.
pub const EVIDENCE_CAPTURE_RUN_2026_10_07: &str = "conductor-0.3.0/chunks/2026-10-07-a-capture-run-records-the-prompt-the-model-received-in-each-drive/evidence";

/// The 2026-10-07 capture run: three drives on one fresh letters-only data dir, against andromeda-pulse
/// `f70be92`. A capture, never a series: its grades are observations.
pub const CAPTURE_RUN_2026_10_07: [Drive; 3] = [
    Drive {
        label: "d1",
        file: "rm-capture-d1.txt",
        sha256: "e9262bc5988dedb6e8b3f83c2df30cd77efe132805a4ca9f6205f085d9d3b16f",
    },
    Drive {
        label: "d2",
        file: "rm-capture-d2.txt",
        sha256: "775b1126da7e13e65575c18dbbf768f83854aa805687c2ecf39e88d68d8a415e",
    },
    Drive {
        label: "d3",
        file: "rm-capture-d3.txt",
        sha256: "0a83022a2dd68a5d4760a02bc25869989ad75247c8f0df11dd96a487e79d6988",
    },
];
