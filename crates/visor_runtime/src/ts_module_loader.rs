use anyhow::anyhow;
use deno_ast::MediaType;
use deno_ast::ParseParams;
use deno_core::futures::FutureExt;

pub struct TsModuleLoader;

impl deno_core::ModuleLoader for TsModuleLoader {
    fn resolve(
        &self,
        specifier: &str,
        referrer: &str,
        _kind: deno_core::ResolutionKind,
    ) -> Result<deno_core::ModuleSpecifier, deno_core::anyhow::Error> {
        deno_core::resolve_import(specifier, referrer).map_err(|error| error.into())
    }

    fn load(
        &self,
        module_specifier: &deno_core::ModuleSpecifier,
        _maybe_referrer: Option<&deno_core::ModuleSpecifier>,
        _is_dyn_import: bool,
        _requested_module_type: deno_core::RequestedModuleType,
    ) -> deno_core::ModuleLoadResponse {
        let module_specifier = module_specifier.clone();

        deno_core::ModuleLoadResponse::Async(
            async move {
                let path = module_specifier.to_file_path().map_err(|_| {
                    anyhow!(
                        "Import error: could not resolve file path: {}",
                        module_specifier
                    )
                })?;

                let media_type = MediaType::from_path(&path);

                let (module_type, should_transpile) = match MediaType::from_path(&path) {
                    MediaType::JavaScript | MediaType::Mjs | MediaType::Cjs => {
                        (deno_core::ModuleType::JavaScript, false)
                    }
                    MediaType::Jsx => (deno_core::ModuleType::JavaScript, true),
                    MediaType::TypeScript
                    | MediaType::Mts
                    | MediaType::Cts
                    | MediaType::Dts
                    | MediaType::Dmts
                    | MediaType::Dcts
                    | MediaType::Tsx => (deno_core::ModuleType::JavaScript, true),
                    MediaType::Json => (deno_core::ModuleType::Json, false),
                    _ => panic!("Unknown extension {:?}", path.extension()),
                };

                let code = std::fs::read_to_string(&path)?;

                let module_source_code = if should_transpile {
                    let parsed = deno_ast::parse_module(ParseParams {
                        specifier: module_specifier.clone(),
                        text: code.into(),
                        media_type,
                        capture_tokens: false,
                        scope_analysis: false,
                        maybe_syntax: None,
                    })?;

                    let source = parsed
                        .transpile(&Default::default(), &Default::default())?
                        .into_source()
                        .source;

                    deno_core::ModuleSourceCode::Bytes(source.into_boxed_slice().into())
                } else {
                    deno_core::ModuleSourceCode::String(code.into())
                };

                let module = deno_core::ModuleSource::new(
                    module_type,
                    module_source_code,
                    &module_specifier,
                    None,
                );

                Ok(module)
            }
            .boxed_local(),
        )
    }
}
