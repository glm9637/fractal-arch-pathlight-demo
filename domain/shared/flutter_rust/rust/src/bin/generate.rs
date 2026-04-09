use quote::ToTokens;
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::{env, fs};
use walkdir::WalkDir;

// --- Pass 1 Types ---
struct StructInfo {
    def: syn::ItemStruct,
    module_path: String,
}

fn main() {
    let root_env = env::var("MONOREPO_ROOT").unwrap_or_else(|_| "../../../".to_string());
    let monorepo_root = Path::new(&root_env);
    let domain_root = monorepo_root.join("domain");
    let gateway_dir = monorepo_root.join("domain/shared/flutter_rust/rust/src/api/auto_gateway");

    setup_directories(&gateway_dir);

    let mut file_contents: HashMap<(String, String), String> = HashMap::new();
    let mut domains: HashSet<String> = HashSet::new();
    // THE GLOBAL DATABASE
    let mut global_structs: HashMap<String, StructInfo> = HashMap::new();

    if let Ok(entries) = fs::read_dir(domain_root) {
        for entry in entries.flatten() {
            let path = entry.path();
            if !path.is_dir() {
                continue;
            }

            let domain_name = entry.file_name().to_string_lossy().to_string();
            if domain_name == "shared" || domain_name.starts_with('.') {
                continue;
            }

            let core_path = path.join("frontend/core");
            let src_path = core_path.join("src");
            let cargo_path = core_path.join("Cargo.toml");

            if src_path.exists() {
                let crate_name = extract_crate_name(&cargo_path)
                    .unwrap_or_else(|| format!("{}_frontend_core", domain_name));

                // --- PASS 1: DISCOVER ALL STRUCTS ---
                let files: Vec<PathBuf> = WalkDir::new(&src_path)
                    .into_iter()
                    .filter_map(|e| e.ok())
                    .map(|e| e.path().to_path_buf())
                    .filter(|p| p.extension().map_or(false, |ext| ext == "rs"))
                    .collect();

                for file_path in &files {
                    let module_path = extract_module_path(&src_path, file_path, &crate_name);
                    if let Ok(content) = fs::read_to_string(file_path) {
                        if let Ok(ast) = syn::parse_file(&content) {
                            for item in ast.items {
                                if let syn::Item::Struct(s) = item {
                                    global_structs.insert(
                                        s.ident.to_string(),
                                        StructInfo {
                                            def: s,
                                            module_path: module_path.clone(),
                                        },
                                    );
                                }
                            }
                        }
                    }
                }

                // --- PASS 2: PROCESS IMPLS ---
                for file_path in &files {
                    let module_path = extract_module_path(&src_path, file_path, &crate_name);
                    process_impls(
                        file_path,
                        &domain_name,
                        &module_path,
                        &global_structs,
                        &mut file_contents,
                        &mut domains,
                    );
                }
            }
        }
    }

    write_generated_files(&gateway_dir, file_contents, domains);
}

fn process_impls(
    path: &Path,
    domain: &str,
    module_path: &str,
    global_structs: &HashMap<String, StructInfo>,
    file_contents: &mut HashMap<(String, String), String>,
    domains: &mut HashSet<String>,
) {
    let content = fs::read_to_string(path).unwrap();
    let ast = syn::parse_file(&content).unwrap();

    for item in ast.items {
        if let syn::Item::Impl(item_impl) = item {
            if let Some((file_type, mut code, needs_mirror, config_to_mirror)) =
                process_impl_block(domain, module_path, &item_impl)
            {
                if let Some(cfg_name) = config_to_mirror {
                    if let Some(info) = global_structs.get(&cfg_name) {
                        let mirror =
                            generate_struct_mirror(&info.module_path, &cfg_name, &info.def);
                        code = format!("{}\n{}", mirror, code);
                    }
                }

                // 2. Mirror the SELF if required (e.g. Commands/Selectors)
                if needs_mirror {
                    let type_path = match &*item_impl.self_ty {
                        syn::Type::Path(tp) => tp,
                        _ => continue,
                    };
                    let struct_name = type_path.path.segments.last().unwrap().ident.to_string();

                    if let Some(info) = global_structs.get(&struct_name) {
                        let mirror =
                            generate_struct_mirror(&info.module_path, &struct_name, &info.def);
                        code = format!("{}\n{}", mirror, code);
                    }
                }

                domains.insert(domain.to_string());
                let key = (domain.to_string(), file_type);
                file_contents.entry(key).or_default().push_str(&code);
            }
        }
    }
}

// --- HELPER TO GENERATE THE FRB MIRROR BLOCK ---
fn generate_struct_mirror(
    module_path: &str,
    struct_name: &str,
    struct_def: &syn::ItemStruct,
) -> String {
    let mut mirror_def = struct_def.clone();
    // FRB needs the mirrored struct to have a different name internally, usually prefixed with _
    mirror_def.ident = syn::Ident::new(&format!("_{}", struct_name), mirror_def.ident.span());
    mirror_def.attrs.clear();

    format!(
        "\npub use {}::{};\n\n#[flutter_rust_bridge::frb(mirror({}))]\n{}\n",
        module_path,
        struct_name,
        struct_name,
        mirror_def.to_token_stream().to_string()
    )
}

fn process_impl_block(
    domain: &str,
    module_path: &str,
    item_impl: &syn::ItemImpl,
) -> Option<(String, String, bool, Option<String>)> {
    let trait_item = item_impl.trait_.as_ref()?;
    let trait_segment = trait_item.1.segments.last()?;
    let trait_name = trait_segment.ident.to_string();

    let type_path = match &*item_impl.self_ty {
        syn::Type::Path(tp) => tp,
        _ => return None,
    };
    let struct_name = type_path.path.segments.last()?.ident.to_string();

    let config_struct = if let syn::PathArguments::AngleBracketed(args) = &trait_segment.arguments {
        args.args.first().and_then(|arg| {
            if let syn::GenericArgument::Type(syn::Type::Path(tp)) = arg {
                Some(tp.path.segments.last()?.ident.to_string())
            } else {
                None
            }
        })
    } else {
        None
    };

    match trait_name.as_str() {
        "Selectable" => Some((
            "selector".into(),
            generate_selector_glue(domain, &struct_name),
            true,
            None,
        )),
        "Command" => Some((
            "command".into(),
            generate_command_glue(domain, &struct_name),
            true,
            None,
        )),
        "SystemInit" => Some((
            "init".into(),
            generate_init_glue(domain, module_path, &struct_name, config_struct.as_deref()),
            false,
            config_struct,
        )),
        "SystemDispose" => Some((
            "dispose".into(),
            generate_dispose_glue(domain, module_path, &struct_name),
            false,
            None,
        )),
        _ => None,
    }
}

fn generate_init_glue(
    domain: &str,
    module_path: &str,
    struct_name: &str,
    config_name: Option<&str>,
) -> String {
    let fn_name = format!("init_{}_system", domain);

    match config_name {
        Some(cfg) => format!(
            r#"
pub fn {fn_name}(config: {cfg}) -> anyhow::Result<()> {{
    <{module_path}::{struct_name} as state_machine::lifetime::SystemInit<{cfg}>>::init_system(config)
}}
"#
        ),
        None => format!(
            r#"
pub fn {fn_name}() -> anyhow::Result<()> {{
    <{module_path}::{struct_name} as state_machine::lifetime::SystemInit>::init_system()
}}
"#
        ),
    }
}

fn generate_dispose_glue(domain: &str, module_path: &str, struct_name: &str) -> String {
    let fn_name = format!("dispose_{}_system", domain);
    format!(
        r#"
pub fn {fn_name}() -> anyhow::Result<()> {{
    <{module_path}::{struct_name} as state_machine::lifetime::SystemDispose>::dispose_system()
}}
"#
    )
}

fn generate_selector_glue(domain: &str, struct_name: &str) -> String {
    let fn_name = to_snake_case(struct_name);

    // We update the signature to `-> anyhow::Result<()>`
    // and map the FRB add() error so it perfectly matches your trait.
    format!(
        r#"
struct {struct_name}FrbSink(crate::frb_generated::StreamSink<{struct_name}>);

impl state_machine::selector::DataSink<{struct_name}> for {struct_name}FrbSink {{
    fn send(&self, state: {struct_name}) -> anyhow::Result<()> {{
        return self.0.add(state).map_err(|_| anyhow::anyhow!("Dart StreamSink closed or failed"))
    }}
}}

pub async fn watch_{fn_name}(sink: crate::frb_generated::StreamSink<{struct_name}>) {{
    let emitter = Box::new({struct_name}FrbSink(sink));
    {domain}_frontend_core::system::get_engine().add_selector(emitter).await;
}}
"#
    )
}

fn generate_command_glue(domain: &str, struct_name: &str) -> String {
    let fn_name = to_snake_case(struct_name);
    format!(
        r#"
pub async fn dispatch_{fn_name}(command: {struct_name}) -> anyhow::Result<()> {{
    {domain}_frontend_core::system::get_engine().dispatch(command).await 
}}
"#
    )
}

fn write_generated_files(
    gateway_dir: &Path,
    file_contents: HashMap<(String, String), String>,
    domains: HashSet<String>,
) {
    let mut main_mod = String::new();
    for domain in domains {
        main_mod.push_str(&format!("pub mod {};\n", domain));
        let mut lifetime_code = String::new();
        let domain_dir = gateway_dir.join(&domain);
        fs::create_dir_all(&domain_dir).expect("Failed to create domain dir");

        let mut domain_mod = String::new();

        if let Some(content) = file_contents.get(&(domain.clone(), "selector".to_string())) {
            domain_mod.push_str("pub mod selector;\n");
            fs::write(
                domain_dir.join("selector.rs"),
                format!("// AUTO-GENERATED\n{}", content),
            )
            .expect("Failed to write selector.rs");
        }

        if let Some(content) = file_contents.get(&(domain.clone(), "command".to_string())) {
            domain_mod.push_str("pub mod command;\n");
            fs::write(
                domain_dir.join("command.rs"),
                format!("// AUTO-GENERATED\n{}", content),
            )
            .expect("Failed to write command.rs");
        }
        if let Some(content) = file_contents.get(&(domain.clone(), "init".to_string())) {
            if lifetime_code.is_empty() {
                domain_mod.push_str("pub mod lifetime;\n");
            } else {
                lifetime_code.push_str("\n");
            }
            lifetime_code.push_str(content);
        }
        if let Some(content) = file_contents.get(&(domain.clone(), "dispose".to_string())) {
            if lifetime_code.is_empty() {
                domain_mod.push_str("pub mod lifetime;\n");
            } else {
                lifetime_code.push_str("\n");
            }
            lifetime_code.push_str(content);
        }
        if !lifetime_code.is_empty() {
            fs::write(
                domain_dir.join("lifetime.rs"),
                format!("// AUTO-GENERATED\n{}", lifetime_code),
            )
            .expect("Failed to write lifetime.rs");
        }
        fs::write(domain_dir.join("mod.rs"), domain_mod).expect("Failed to write domain/mod.rs");
    }
    fs::write(gateway_dir.join("mod.rs"), main_mod).expect("Failed to write auto_gateway/mod.rs");
}

fn to_snake_case(camel_case: &str) -> String {
    let mut snake = String::new();
    for (i, ch) in camel_case.char_indices() {
        if i > 0 && ch.is_uppercase() {
            snake.push('_');
        }
        snake.push(ch.to_ascii_lowercase());
    }
    snake
}

fn extract_crate_name(toml_path: &Path) -> Option<String> {
    let content = fs::read_to_string(toml_path).ok()?;
    let mut in_package = false;

    for line in content.lines() {
        let line = line.trim();

        if line == "[package]" {
            in_package = true;
            continue;
        }

        if in_package && line.starts_with('[') {
            break;
        }

        if in_package && line.starts_with("name") {
            let parts: Vec<&str> = line.split('=').collect();
            if parts.len() == 2 {
                // Strip the quotes and whitespace
                let name = parts[1].trim().trim_matches('"').trim_matches('\'');
                return Some(name.to_string());
            }
        }
    }
    None
}

fn extract_module_path(src_path: &Path, file_path: &Path, crate_name: &str) -> String {
    let relative_path = file_path.strip_prefix(src_path).unwrap_or(file_path);
    // Normalize slashes for cross-platform compatibility
    let path_str = relative_path.to_string_lossy().replace('\\', "/");

    let without_ext = path_str.strip_suffix(".rs").unwrap_or(&path_str);

    let rust_module_path = if without_ext == "lib" || without_ext == "mod" {
        String::new()
    } else if without_ext.ends_with("/mod") {
        without_ext.strip_suffix("/mod").unwrap().replace('/', "::")
    } else {
        without_ext.replace('/', "::")
    };

    if rust_module_path.is_empty() {
        crate_name.to_string()
    } else {
        format!("{}::{}", crate_name, rust_module_path)
    }
}

fn setup_directories(gateway_dir: &Path) {
    if gateway_dir.exists() {
        fs::remove_dir_all(gateway_dir).expect("Failed to clean auto_gateway dir");
    }
    fs::create_dir_all(gateway_dir).expect("Failed to create auto_gateway dir");
}
