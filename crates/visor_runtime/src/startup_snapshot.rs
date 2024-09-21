use std::sync::OnceLock;

use deno_core::{Extension, OpDecl};

pub static STARTUP_SNAPSHOT_CELL: OnceLock<StartupSnapshot> = OnceLock::new();

pub struct StartupSnapshot {
    pub(crate) ops: Vec<OpDecl>,
    pub(crate) snapshot: Vec<u8>,
}

// Note: This is required for shared access across threads
// Should be safe given the cell value is never mutated
unsafe impl Send for StartupSnapshot {}
unsafe impl Sync for StartupSnapshot {}

impl StartupSnapshot {
    pub fn new(extensions: Vec<Extension>) -> Self {
        let ops: Vec<_> = extensions
            .iter()
            .flat_map(|extension| extension.ops.to_vec())
            .collect();

        let snapshot = deno_core::snapshot::create_snapshot(
            deno_core::snapshot::CreateSnapshotOptions {
                cargo_manifest_dir: env!("CARGO_MANIFEST_DIR"),
                startup_snapshot: None,
                skip_op_registration: false,
                extensions,
                // TODO: try use TsModuleLoader for transpiling extensions?
                extension_transpiler: None,
                with_runtime_cb: None,
            },
            None,
        )
        .expect("Unexpected: could not create snapshot");

        Self {
            ops,
            snapshot: snapshot.output.to_vec(),
        }
    }
}
