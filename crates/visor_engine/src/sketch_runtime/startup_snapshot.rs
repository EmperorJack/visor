use std::{rc::Rc, sync::OnceLock};

use deno_core::Extension;

use crate::sketch_runtime::ts_module_loader::maybe_transpile_source;

pub(crate) static STARTUP_SNAPSHOT_CELL: OnceLock<StartupSnapshot> = OnceLock::new();

pub(crate) struct StartupSnapshot {
    pub(crate) snapshot: Vec<u8>,
}

// Note: This is required for shared access across threads
// Should be safe given the cell value is never mutated
unsafe impl Send for StartupSnapshot {}
unsafe impl Sync for StartupSnapshot {}

impl StartupSnapshot {
    pub(crate) fn new(extensions: Vec<Extension>) -> Self {
        let snapshot = deno_core::snapshot::create_snapshot(
            deno_core::snapshot::CreateSnapshotOptions {
                cargo_manifest_dir: env!("CARGO_MANIFEST_DIR"),
                startup_snapshot: None,
                skip_op_registration: false,
                extensions,
                extension_transpiler: Some(Rc::new(maybe_transpile_source)),
                with_runtime_cb: None,
            },
            None,
        )
        .expect("Unexpected: could not create snapshot");

        Self {
            snapshot: snapshot.output.to_vec(),
        }
    }
}
