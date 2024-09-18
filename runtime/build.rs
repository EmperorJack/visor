use deno_core::extension;
use std::{fs::File, io::Write, path::PathBuf};

extension!(
    draw_extension,
    esm_entry_point = "visor:draw",
    esm = [
        dir "src",
        "visor:draw" = "visor-draw.js",
    ],
);

fn main() {
    let output_dir = PathBuf::from(
        std::env::var_os("OUT_DIR")
            .expect("Unexpected: could not fetch environment variable OUT_DIR"),
    );
    let snapshot_path = output_dir.join("RUNTIME_SNAPSHOT.bin");

    let snapshot = deno_core::snapshot::create_snapshot(
        deno_core::snapshot::CreateSnapshotOptions {
            cargo_manifest_dir: env!("CARGO_MANIFEST_DIR"),
            startup_snapshot: None,
            skip_op_registration: false,
            extensions: vec![draw_extension::init_ops_and_esm()],
            // TODO: use TsModuleLoader for transpiling runtime.js?
            extension_transpiler: None,
            with_runtime_cb: None,
        },
        None,
    )
    .expect("Unexpected: could not create snapshot");

    let mut file = File::create(snapshot_path.clone())
        .expect("Unexpected: could not open file to write snapshot");

    file.write_all(&snapshot.output)
        .expect("Unexpected: could not write snapshot to file");

    println!("Wrote runtime snapshot to {:?}", snapshot_path);
}
