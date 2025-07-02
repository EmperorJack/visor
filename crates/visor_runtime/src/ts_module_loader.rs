use deno_ast::{EmitOptions, MediaType, ParseParams};
use deno_core::{
    ModuleLoadResponse, ModuleLoader, ModuleSource, ModuleSourceCode, ModuleSpecifier, ModuleType,
    RequestedModuleType, ResolutionKind, error::ModuleLoaderError, futures::FutureExt, url::Url,
};
use deno_error::JsErrorBox;

pub struct TsModuleLoader;

impl ModuleLoader for TsModuleLoader {
    fn resolve(
        &self,
        specifier: &str,
        referrer: &str,
        _kind: ResolutionKind,
    ) -> Result<Url, ModuleLoaderError> {
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
                    JsErrorBox::generic(format!(
                        "Could not resolve file path for import \"{}\"",
                        module_specifier
                    ))
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
                            Some(extension) => JsErrorBox::generic(format!(
                                "Unknown file extension \".{}\" for import \"{}\"",
                                extension,
                                path.display(),
                            ))
                            .into(),
                            None => JsErrorBox::generic(format!(
                                "Missing file extension for import \"{}\"",
                                path.display()
                            ))
                            .into(),
                        });
                    }
                };

                let code = std::fs::read_to_string(&path).map_err(|_| {
                    JsErrorBox::generic(format!(
                        "No such file found for import \"{}\"",
                        path.display()
                    ))
                })?;

                let code = if should_transpile {
                    let parsed = deno_ast::parse_module(ParseParams {
                        specifier: module_specifier.clone(),
                        text: code.into(),
                        media_type,
                        capture_tokens: false,
                        scope_analysis: false,
                        maybe_syntax: None,
                    })
                    .map_err(JsErrorBox::from_err)?;

                    let source = parsed
                        .transpile(
                            &Default::default(),
                            &Default::default(),
                            &EmitOptions::default(),
                        )
                        .map_err(JsErrorBox::from_err)?
                        .into_source();

                    source.text
                } else {
                    code
                };

                let module = ModuleSource::new(
                    module_type,
                    ModuleSourceCode::String(code.into()),
                    &module_specifier,
                    None,
                );

                Ok(module)
            }
            .boxed_local(),
        )
    }
}
