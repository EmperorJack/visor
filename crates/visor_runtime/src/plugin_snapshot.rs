use std::sync::OnceLock;

use deno_core::OpDecl;
use visor_plugin::plugin::Plugin;

pub static PLUGIN_SNAPSHOT_CELL: OnceLock<PluginSnapshot> = OnceLock::new();

pub struct PluginSnapshot {
    pub(crate) ops: Vec<OpDecl>,
    pub(crate) snapshot: Vec<u8>,
}

// Note: This is required for shared access across threads
// Should be safe given the cell value is never mutated
unsafe impl Send for PluginSnapshot {}
unsafe impl Sync for PluginSnapshot {}

impl PluginSnapshot {
    pub fn new(plugins: &[Box<dyn Plugin>]) -> Self {
        let plugin_extensions: Vec<_> = plugins.iter().map(|plugin| plugin.extension()).collect();

        let ops: Vec<_> = plugin_extensions
            .iter()
            .flat_map(|extension| extension.ops.to_vec())
            .collect();

        let snapshot = deno_core::snapshot::create_snapshot(
            deno_core::snapshot::CreateSnapshotOptions {
                cargo_manifest_dir: env!("CARGO_MANIFEST_DIR"),
                startup_snapshot: None,
                skip_op_registration: false,
                extensions: plugin_extensions,
                // TODO: try use TsModuleLoader for transpiling plugins?
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
