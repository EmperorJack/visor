use std::path::Path;

use deno_ast::{EmitOptions, MediaType, ParseParams};
use deno_core::{
    ModuleCodeString, ModuleLoadResponse, ModuleLoader, ModuleName, ModuleSource, ModuleSourceCode,
    ModuleSpecifier, ModuleType, RequestedModuleType, ResolutionKind, SourceMapData,
    error::ModuleLoaderError, url::Url,
};
use deno_error::JsErrorBox;

pub(crate) struct TsModuleLoader;

impl ModuleLoader for TsModuleLoader {
    fn resolve(
        &self,
        specifier: &str,
        referrer: &str,
        _kind: ResolutionKind,
    ) -> Result<ModuleSpecifier, ModuleLoaderError> {
        Ok(deno_core::resolve_import(specifier, referrer)?)
    }

    fn load(
        &self,
        module_specifier: &ModuleSpecifier,
        _maybe_referrer: Option<&ModuleSpecifier>,
        _is_dyn_import: bool,
        _requested_module_type: RequestedModuleType,
    ) -> ModuleLoadResponse {
        let module_specifier = module_specifier.clone();

        let module_load = move || {
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
        };

        ModuleLoadResponse::Sync(module_load())
    }
}

pub(crate) fn maybe_transpile_source(
    specifier: ModuleName,
    source: ModuleCodeString,
) -> Result<(ModuleCodeString, Option<SourceMapData>), JsErrorBox> {
    let media_type = MediaType::from_path(Path::new(&specifier));

    match media_type {
        MediaType::TypeScript => {}
        MediaType::JavaScript => return Ok((source, None)),
        MediaType::Mjs => return Ok((source, None)),
        _ => panic!(
            "Unsupported media type for snapshotting {} for file {}",
            media_type, specifier
        ),
    }

    let parsed = deno_ast::parse_module(ParseParams {
        specifier: Url::parse(&specifier).expect("Failed to parse module specifier"),
        text: source.into(),
        media_type,
        capture_tokens: false,
        scope_analysis: false,
        maybe_syntax: None,
    })
    .map_err(JsErrorBox::from_err)?;

    let transpiled_source = parsed
        .transpile(
            &Default::default(),
            &Default::default(),
            &EmitOptions::default(),
        )
        .map_err(JsErrorBox::from_err)?
        .into_source();

    Ok((
        transpiled_source.text.into(),
        transpiled_source
            .source_map
            .map(|source_map| source_map.into_bytes().into()),
    ))
}
