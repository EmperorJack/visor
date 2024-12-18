use anyhow::{anyhow, Error};
use deno_ast::{MediaType, ParseParams};
use deno_core::{
    futures::FutureExt, ModuleLoadResponse, ModuleLoader, ModuleSource, ModuleSourceCode,
    ModuleSpecifier, ModuleType, RequestedModuleType, ResolutionKind,
};

pub struct TsModuleLoader;

impl ModuleLoader for TsModuleLoader {
    fn resolve(
        &self,
        specifier: &str,
        referrer: &str,
        _kind: ResolutionKind,
    ) -> Result<ModuleSpecifier, Error> {
        deno_core::resolve_import(specifier, referrer).map_err(|error| error.into())
    }

    fn load(
        &self,
        module_specifier: &ModuleSpecifier,
        _maybe_referrer: Option<&ModuleSpecifier>,
        _is_dyn_import: bool,
        _requested_module_type: RequestedModuleType,
    ) -> ModuleLoadResponse {
        let module_specifier = module_specifier.clone();

        ModuleLoadResponse::Async(
            async move {
                let path = module_specifier.to_file_path().map_err(|_| {
                    anyhow!(
                        "Could not resolve file path for import \"{}\"",
                        module_specifier
                    )
                })?;

                let media_type = MediaType::from_path(&path);

                let (module_type, should_transpile) = match MediaType::from_path(&path) {
                    MediaType::JavaScript | MediaType::Mjs | MediaType::Cjs => {
                        (ModuleType::JavaScript, false)
                    }
                    MediaType::Jsx => (ModuleType::JavaScript, true),
                    MediaType::TypeScript
                    | MediaType::Mts
                    | MediaType::Cts
                    | MediaType::Dts
                    | MediaType::Dmts
                    | MediaType::Dcts
                    | MediaType::Tsx => (ModuleType::JavaScript, true),
                    MediaType::Json => (ModuleType::Json, false),
                    _ => {
                        let extension = path.extension().and_then(|extension| extension.to_str());

                        return Err(match extension {
                            Some(extension) => {
                                anyhow!(
                                    "Unknown file extension \".{}\" for import \"{}\"",
                                    extension,
                                    path.display(),
                                )
                            }
                            None => {
                                anyhow!("Missing file extension for import \"{}\"", path.display())
                            }
                        });
                    }
                };

                let code = std::fs::read_to_string(&path)
                    .map_err(|_| anyhow!("No such file found for import \"{}\"", path.display()))?;

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

                    ModuleSourceCode::Bytes(source.into_boxed_slice().into())
                } else {
                    ModuleSourceCode::String(code.into())
                };

                let module =
                    ModuleSource::new(module_type, module_source_code, &module_specifier, None);

                Ok(module)
            }
            .boxed_local(),
        )
    }
}
