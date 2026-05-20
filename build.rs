/**
 * Copyright 2025-present Coinbase Global, Inc.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *  http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
use convert_case::Casing;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;

// Configuration constants
const MODELS_DIR: &str = "api_spec/types/src/models";
const GENERATED_DIR: &str = "src/generated";

// Filename prefixes stripped when deriving module names from OpenAPI model files
const FILENAME_PREFIXES: &[&str] = &[
    "coinbase_period_custody_period_api_period_",
    "coinbase_period_public_rest_api_period_",
    "coinbase_brokerage_proxy_events_materialized_api_",
    "coinbase_public_rest_api_",
    "coinbase_custody_api_",
    "prime_restapi_",
    "public_rest_api_",
    "prime_beta_",
    "beta_prime_",
    "beta_",
];

// Type-name prefixes stripped when simplifying generated struct/enum names
const TYPE_PREFIXES: &[&str] = &[
    "CoinbasePeriodCustodyPeriodApiPeriod",
    "CoinbasePeriodPublicRestApiPeriod",
    "CoinbaseBrokerageProxyEventsMaterializedApi",
    "CoinbasePublicRestApi",
    "CoinbaseCustodyApi",
    "PrimeRestapi",
    "PublicRestApi",
    "PrimeBeta",
    "BetaPrime",
    "Beta",
];

// Type overrides for simplifying complex type names
const TYPE_OVERRIDES: &[(&str, &str)] = &[
    (
        "CoinbasePeriodCustodyPeriodApiPeriodActivityType",
        "CustodyActivityType",
    ),
    (
        "CoinbasePeriodPublicRestApiPeriodActivityType",
        "PrimeActivityType",
    ),
    ("CoinbaseCustodyApiActivityType", "CustodyActivityType"),
    ("CoinbasePublicRestApiActivityType", "PrimeActivityType"),
    ("ActivityType", "PrimeActivityType"),
];

fn main() {
    println!("cargo:rerun-if-changed=api_spec/prime-public-api-spec.json");
    println!("cargo:rerun-if-changed=api_spec/types/src/models");

    if !Path::new(MODELS_DIR).exists() {
        eprintln!(
            "Warning: OpenAPI Generator CLI output not found at {}",
            MODELS_DIR
        );
        eprintln!("Please run 'make generate-types' or 'make all' first");
        generate_fallback_types();
        return;
    }

    if let Err(e) = process_generated_types() {
        eprintln!("Error processing generated types: {}", e);
        generate_fallback_types();
    }
}

fn process_generated_types() -> Result<(), Box<dyn std::error::Error>> {
    let generated_dir = Path::new(GENERATED_DIR);
    if !generated_dir.exists() {
        fs::create_dir_all(generated_dir)?;
    }

    let type_overrides = build_type_overrides();
    let type_to_module = build_type_mapping(&type_overrides)?;
    let mut written_modules = HashSet::new();
    let mut mod_rs_content = create_mod_rs_header();

    process_model_files(
        &type_overrides,
        &type_to_module,
        &mut written_modules,
        &mut mod_rs_content,
    )?;

    write_output_files(&mod_rs_content)?;

    // Fix PrimeActivityType references in custody-related files
    fix_custody_activity_type_references()?;

    println!("cargo:rerun-if-changed=src/generated_types.rs");
    println!("cargo:rerun-if-changed=src/generated");
    Ok(())
}

fn build_type_overrides() -> HashMap<&'static str, &'static str> {
    TYPE_OVERRIDES.iter().cloned().collect()
}

fn should_skip_model(filename: &str) -> bool {
    filename == "google_protobuf_any" || filename == "google_rpc_status"
}

fn build_type_mapping(
    type_overrides: &HashMap<&str, &str>,
) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
    let mut type_to_module = HashMap::new();
    let models_dir = Path::new(MODELS_DIR);

    for entry in fs::read_dir(models_dir)? {
        let entry = entry?;
        if let Some(extension) = entry.path().extension() {
            if extension == "rs" {
                let path = entry.path();
                let filename = path.file_stem().unwrap().to_str().unwrap();
                if !should_skip_model(filename) {
                    process_type_mapping_file(&path, type_overrides, &mut type_to_module)?;
                }
            }
        }
    }

    Ok(type_to_module)
}

fn process_type_mapping_file(
    path: &Path,
    type_overrides: &HashMap<&str, &str>,
    type_to_module: &mut HashMap<String, String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let original_filename = path.file_stem().unwrap().to_str().unwrap();
    let simplified_name = simplify_filename(original_filename);
    let content = fs::read_to_string(path)?;

    let re = Regex::new(r"pub (struct|enum|union) ([A-Za-z0-9_]+)")?;
    for cap in re.captures_iter(&content) {
        let original_type_name = cap[2].to_string();
        let simplified_type_name = apply_type_override(&original_type_name, type_overrides);
        let module_name = if type_overrides.contains_key(original_type_name.as_str()) {
            simplified_type_name.to_case(convert_case::Case::Snake)
        } else {
            simplified_name.clone()
        };

        println!(
            "Mapping type {} -> {} (module: {})",
            original_type_name, simplified_type_name, module_name
        );
        type_to_module.insert(simplified_type_name, module_name);
    }

    Ok(())
}

fn simplify_filename(filename: &str) -> String {
    let mut simplified = filename.to_string();
    for prefix in FILENAME_PREFIXES {
        simplified = simplified.replace(prefix, "");
    }
    simplified
}

fn strip_type_prefixes(type_name: &str) -> String {
    let mut simplified = type_name.to_string();
    loop {
        let matching_prefix = TYPE_PREFIXES
            .iter()
            .filter(|prefix| simplified.starts_with(**prefix) && simplified.len() > prefix.len())
            .max_by_key(|prefix| prefix.len());

        match matching_prefix {
            Some(prefix) => simplified = simplified[prefix.len()..].to_string(),
            None => break,
        }
    }
    simplified
}

fn strip_type_prefixes_from_identifiers(content: &str) -> String {
    let identifier_re = Regex::new(r"\b([A-Z][A-Za-z0-9_]*)\b").unwrap();
    identifier_re
        .replace_all(content, |caps: &regex::Captures| {
            let stripped = strip_type_prefixes(&caps[1]);
            if stripped.is_empty() {
                caps[1].to_string()
            } else {
                stripped
            }
        })
        .into_owned()
}

fn apply_type_override(type_name: &str, type_overrides: &HashMap<&str, &str>) -> String {
    type_overrides
        .get(type_name)
        .map(|s| s.to_string())
        .unwrap_or_else(|| strip_type_prefixes(type_name))
}

fn create_mod_rs_header() -> String {
    let mut content = String::new();
    content.push_str("/* Auto-generated types from OpenAPI specification\n");
    content.push_str("* DO NOT EDIT - This file is generated automatically\n");
    content.push_str("* Run 'make generate-types' to regenerate this file\n\n");
    content.push_str(" * Copyright 2025-present Coinbase Global, Inc.\n");
    content.push_str(" *\n");
    content.push_str(" * This file is generated by Openapi Generator https://github.com/openapitools/openapi-generator\n");
    content.push_str(" *\n");
    content.push_str(" * Licensed under the Apache License, Version 2.0 (the \"License\");\n");
    content.push_str(" * you may not use this file except in compliance with the License.\n");
    content.push_str(" * You may obtain a copy of the License at\n");
    content.push_str(" *\n");
    content.push_str(" *  http://www.apache.org/licenses/LICENSE-2.0\n");
    content.push_str(" *\n");
    content.push_str(" * Unless required by applicable law or agreed to in writing, software\n");
    content.push_str(" * distributed under the License is distributed on an \"AS IS\" BASIS,\n");
    content
        .push_str(" * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.\n");
    content.push_str(" * See the License for the specific language governing permissions and\n");
    content.push_str(" * limitations under the License.\n");
    content.push_str(" *\n");
    content.push_str(" *  Do not edit the class manually.\n");
    content.push_str(" */\n\n");
    content
}

fn process_model_files(
    type_overrides: &HashMap<&str, &str>,
    type_to_module: &HashMap<String, String>,
    written_modules: &mut HashSet<String>,
    mod_rs_content: &mut String,
) -> Result<(), Box<dyn std::error::Error>> {
    let models_dir = Path::new(MODELS_DIR);

    for entry in fs::read_dir(models_dir)? {
        let entry = entry?;
        if let Some(extension) = entry.path().extension() {
            if extension == "rs" {
                let path = entry.path();
                let filename = path.file_stem().unwrap().to_str().unwrap();
                if !should_skip_model(filename) {
                    process_single_model_file(
                        &path,
                        type_overrides,
                        type_to_module,
                        written_modules,
                        mod_rs_content,
                    )?;
                }
            }
        }
    }

    Ok(())
}

fn process_single_model_file(
    path: &Path,
    type_overrides: &HashMap<&str, &str>,
    type_to_module: &HashMap<String, String>,
    written_modules: &mut HashSet<String>,
    mod_rs_content: &mut String,
) -> Result<(), Box<dyn std::error::Error>> {
    let original_filename = path.file_stem().unwrap().to_str().unwrap();
    let simplified_name = simplify_filename(original_filename);
    let content = fs::read_to_string(path)?;

    let override_defined = find_override_definition(&content, type_overrides);
    let processed_content = process_file_content(&content, type_overrides, original_filename);
    let processed_content_final = replace_qualified_paths(&processed_content, type_to_module);

    let defined_types = find_defined_types(&processed_content_final);
    let referenced_types = find_referenced_types(
        &processed_content_final,
        type_to_module,
        &simplified_name,
        &defined_types,
    );
    let use_statements = build_use_statements(&referenced_types, type_to_module, &defined_types);

    let final_output = build_final_output(&use_statements, &processed_content_final);
    let final_module_name = determine_module_name(
        &override_defined,
        &processed_content_final,
        &simplified_name,
        type_overrides,
    );

    if override_defined.is_some() || !written_modules.contains(&final_module_name) {
        let output_file = Path::new(GENERATED_DIR).join(format!("{}.rs", final_module_name));
        fs::write(&output_file, final_output)?;
        mod_rs_content.push_str(&format!("pub mod {};\n", final_module_name));
        written_modules.insert(final_module_name);
    }

    Ok(())
}

fn find_override_definition(content: &str, type_overrides: &HashMap<&str, &str>) -> Option<String> {
    for (original, override_name) in type_overrides {
        if content.contains(&format!("pub struct {}", original))
            || content.contains(&format!("pub enum {}", original))
            || content.contains(&format!("pub union {}", original))
        {
            return Some(override_name.to_case(convert_case::Case::Snake));
        }
    }
    None
}

fn process_file_content(
    content: &str,
    type_overrides: &HashMap<&str, &str>,
    filename: &str,
) -> String {
    let mut processed_content = String::new();
    let mut skip_openapi_header = false;

    for line in content.lines() {
        // Check if we're entering the OpenAPI Generator header comment block
        if line.trim() == "/*" {
            skip_openapi_header = true;
            continue;
        }

        // Check if we're exiting the OpenAPI Generator header comment block
        if skip_openapi_header && line.trim() == "*/" {
            skip_openapi_header = false;
            continue;
        }

        // Skip all lines while we're inside the OpenAPI Generator header comment block
        if skip_openapi_header {
            continue;
        }

        if should_skip_line(line) {
            continue;
        }

        let mut processed_line = apply_line_transformations(line);
        processed_line = apply_type_overrides_to_line(&processed_line, type_overrides);

        if !processed_line.trim().is_empty() {
            processed_content.push_str(&processed_line);
            processed_content.push_str("\n");
        }
    }

    // Special case: custody activity type uses CustodyActivityType instead of PrimeActivityType
    if (filename == "coinbase_period_custody_period_api_period_activity_type"
        || filename == "coinbase_custody_api_activity_type")
        && processed_content.contains("pub enum PrimeActivityType")
    {
        processed_content =
            processed_content.replace("pub enum PrimeActivityType", "pub enum CustodyActivityType");
        processed_content = processed_content.replace(
            "impl std::fmt::Display for PrimeActivityType",
            "impl std::fmt::Display for CustodyActivityType",
        );
        processed_content = processed_content.replace(
            "impl Default for PrimeActivityType",
            "impl Default for CustodyActivityType",
        );
        processed_content =
            processed_content.replace("-> PrimeActivityType", "-> CustodyActivityType");
    }
    // For all custody files, replace references to PrimeActivityType with CustodyActivityType
    if is_custody_model(filename) {
        processed_content = processed_content.replace("PrimeActivityType", "CustodyActivityType");
    }

    processed_content
}

fn should_skip_line(line: &str) -> bool {
    line.trim().starts_with("pub mod")
        || line.trim().starts_with("use ")
        || line.trim().starts_with("extern crate")
        || line.trim().is_empty()
        || line.trim() == "///"
}

fn apply_line_transformations(line: &str) -> String {
    let transformed = line.replace("models::", "crate::types::generated::generated::");
    strip_type_prefixes_from_identifiers(&transformed)
}

fn is_custody_model(filename: &str) -> bool {
    filename.starts_with("coinbase_period_custody_period_api_period_")
        || filename.starts_with("coinbase_custody_api_")
}

fn apply_type_overrides_to_line(line: &str, type_overrides: &HashMap<&str, &str>) -> String {
    let mut processed_line = line.to_string();

    for (original, override_name) in type_overrides {
        if processed_line.contains(original) {
            println!(
                "Replacing {} with {} in line: {}",
                original,
                override_name,
                line.trim()
            );
        }
        let pattern = format!(r"\b{}\b", regex::escape(original));
        let re = Regex::new(&pattern).unwrap();
        processed_line = re.replace_all(&processed_line, *override_name).to_string();
    }

    processed_line
}

fn replace_qualified_paths(content: &str, type_to_module: &HashMap<String, String>) -> String {
    let mut processed_content = content.to_string();

    for (type_name, module) in type_to_module {
        let fq_path_module = format!(
            "crate::types::generated::generated::{}::{}",
            module, type_name
        );
        let fq_path_direct = format!("crate::types::generated::generated::{}", type_name);

        if processed_content.contains(&fq_path_module) {
            processed_content = processed_content.replace(&fq_path_module, type_name);
        }
        if processed_content.contains(&fq_path_direct) {
            processed_content = processed_content.replace(&fq_path_direct, type_name);
        }
    }

    processed_content
}

fn find_defined_types(content: &str) -> HashSet<String> {
    let mut defined_types = HashSet::new();
    let def_re = Regex::new(r"pub (struct|enum|union) ([A-Za-z0-9_]+)").unwrap();

    for cap in def_re.captures_iter(content) {
        defined_types.insert(cap[2].to_string());
    }

    defined_types
}

fn find_referenced_types(
    content: &str,
    type_to_module: &HashMap<String, String>,
    simplified_name: &str,
    defined_types: &HashSet<String>,
) -> HashSet<String> {
    let mut referenced_types = HashSet::new();
    let known_types: HashSet<_> = type_to_module.keys().collect();

    // Collect enum variant names to exclude them
    let enum_variants = collect_enum_variants(content);

    // Remove documentation comments to prevent false positives
    let content_without_docs = remove_documentation_comments(content);

    // Apply various regex patterns to find actual type references
    find_type_references_in_fields(
        &content_without_docs,
        &known_types,
        type_to_module,
        simplified_name,
        defined_types,
        &enum_variants,
        &mut referenced_types,
    );
    find_type_references_in_generics(
        &content_without_docs,
        &known_types,
        type_to_module,
        simplified_name,
        defined_types,
        &enum_variants,
        &mut referenced_types,
    );
    find_type_references_in_returns(
        &content_without_docs,
        &known_types,
        type_to_module,
        simplified_name,
        defined_types,
        &enum_variants,
        &mut referenced_types,
    );
    find_type_references_in_variables(
        &content_without_docs,
        &known_types,
        type_to_module,
        simplified_name,
        defined_types,
        &enum_variants,
        &mut referenced_types,
    );
    find_type_references_in_parameters(
        &content_without_docs,
        &known_types,
        type_to_module,
        simplified_name,
        defined_types,
        &enum_variants,
        &mut referenced_types,
    );
    find_type_references_in_parameter_lists(
        &content_without_docs,
        &known_types,
        type_to_module,
        simplified_name,
        defined_types,
        &enum_variants,
        &mut referenced_types,
    );
    find_type_references_direct(
        &content_without_docs,
        &known_types,
        type_to_module,
        simplified_name,
        defined_types,
        &enum_variants,
        &mut referenced_types,
    );

    referenced_types
}

fn collect_enum_variants(content: &str) -> HashSet<String> {
    let mut enum_variants = HashSet::new();
    let enum_variant_re = Regex::new(r"Self::([A-Z][A-Za-z0-9_]*)").unwrap();

    for cap in enum_variant_re.captures_iter(content) {
        enum_variants.insert(cap[1].to_string());
    }

    enum_variants
}

fn remove_documentation_comments(content: &str) -> String {
    let doc_comment_re = Regex::new(r"///.*\n|/\*.*?\*/").unwrap();
    doc_comment_re.replace_all(content, "").to_string()
}

fn find_type_references_in_fields(
    content: &str,
    known_types: &HashSet<&String>,
    type_to_module: &HashMap<String, String>,
    simplified_name: &str,
    defined_types: &HashSet<String>,
    enum_variants: &HashSet<String>,
    referenced_types: &mut HashSet<String>,
) {
    let field_re =
        Regex::new(r"pub\s+(?:r#)?[a-zA-Z_][a-zA-Z0-9_]*\s*:\s*([A-Z][A-Za-z0-9_]*)").unwrap();

    for cap in field_re.captures_iter(content) {
        let type_name = cap[1].to_string();
        if should_add_type_reference(
            &type_name,
            known_types,
            type_to_module,
            simplified_name,
            defined_types,
            enum_variants,
        ) {
            referenced_types.insert(type_name);
        }
    }
}

fn find_type_references_in_generics(
    content: &str,
    known_types: &HashSet<&String>,
    type_to_module: &HashMap<String, String>,
    simplified_name: &str,
    defined_types: &HashSet<String>,
    enum_variants: &HashSet<String>,
    referenced_types: &mut HashSet<String>,
) {
    let generic_re = Regex::new(r"<([A-Z][A-Za-z0-9_]*)>").unwrap();

    for cap in generic_re.captures_iter(content) {
        let type_name = cap[1].to_string();
        if should_add_type_reference(
            &type_name,
            known_types,
            type_to_module,
            simplified_name,
            defined_types,
            enum_variants,
        ) {
            referenced_types.insert(type_name);
        }
    }
}

fn find_type_references_in_returns(
    content: &str,
    known_types: &HashSet<&String>,
    type_to_module: &HashMap<String, String>,
    simplified_name: &str,
    defined_types: &HashSet<String>,
    enum_variants: &HashSet<String>,
    referenced_types: &mut HashSet<String>,
) {
    let return_re = Regex::new(r"->\s*([A-Z][A-Za-z0-9_]*)").unwrap();

    for cap in return_re.captures_iter(content) {
        let type_name = cap[1].to_string();
        if should_add_type_reference(
            &type_name,
            known_types,
            type_to_module,
            simplified_name,
            defined_types,
            enum_variants,
        ) {
            referenced_types.insert(type_name);
        }
    }
}

fn find_type_references_in_variables(
    content: &str,
    known_types: &HashSet<&String>,
    type_to_module: &HashMap<String, String>,
    simplified_name: &str,
    defined_types: &HashSet<String>,
    enum_variants: &HashSet<String>,
    referenced_types: &mut HashSet<String>,
) {
    let var_re = Regex::new(r"let\s+[a-zA-Z_][a-zA-Z0-9_]*\s*:\s*([A-Z][A-Za-z0-9_]*)").unwrap();

    for cap in var_re.captures_iter(content) {
        let type_name = cap[1].to_string();
        if should_add_type_reference(
            &type_name,
            known_types,
            type_to_module,
            simplified_name,
            defined_types,
            enum_variants,
        ) {
            referenced_types.insert(type_name);
        }
    }
}

fn find_type_references_in_parameters(
    content: &str,
    known_types: &HashSet<&String>,
    type_to_module: &HashMap<String, String>,
    simplified_name: &str,
    defined_types: &HashSet<String>,
    enum_variants: &HashSet<String>,
    referenced_types: &mut HashSet<String>,
) {
    let param_re = Regex::new(r"\(\s*[a-zA-Z_][a-zA-Z0-9_]*\s*:\s*([A-Z][A-Za-z0-9_]*)").unwrap();

    for cap in param_re.captures_iter(content) {
        let type_name = cap[1].to_string();
        if should_add_type_reference(
            &type_name,
            known_types,
            type_to_module,
            simplified_name,
            defined_types,
            enum_variants,
        ) {
            referenced_types.insert(type_name);
        }
    }
}

fn find_type_references_in_parameter_lists(
    content: &str,
    known_types: &HashSet<&String>,
    type_to_module: &HashMap<String, String>,
    simplified_name: &str,
    defined_types: &HashSet<String>,
    enum_variants: &HashSet<String>,
    referenced_types: &mut HashSet<String>,
) {
    let param_list_re =
        Regex::new(r",\s*[a-zA-Z_][a-zA-Z0-9_]*\s*:\s*([A-Z][A-Za-z0-9_]*)").unwrap();

    for cap in param_list_re.captures_iter(content) {
        let type_name = cap[1].to_string();
        if should_add_type_reference(
            &type_name,
            known_types,
            type_to_module,
            simplified_name,
            defined_types,
            enum_variants,
        ) {
            referenced_types.insert(type_name);
        }
    }
}

fn find_type_references_direct(
    content: &str,
    known_types: &HashSet<&String>,
    type_to_module: &HashMap<String, String>,
    simplified_name: &str,
    defined_types: &HashSet<String>,
    enum_variants: &HashSet<String>,
    referenced_types: &mut HashSet<String>,
) {
    let direct_re = Regex::new(r"\b([A-Z][A-Za-z0-9_]*)\b").unwrap();

    for cap in direct_re.captures_iter(content) {
        let type_name = cap[1].to_string();
        if should_add_type_reference(
            &type_name,
            known_types,
            type_to_module,
            simplified_name,
            defined_types,
            enum_variants,
        ) && !referenced_types.contains(&type_name)
        {
            let context_re = Regex::new(&format!(
                r":\s*{}\b|<\s*{}\s*>|->\s*{}\b",
                regex::escape(&type_name),
                regex::escape(&type_name),
                regex::escape(&type_name)
            ))
            .unwrap();
            if context_re.is_match(content) {
                referenced_types.insert(type_name);
            }
        }
    }
}

fn should_add_type_reference(
    type_name: &str,
    known_types: &HashSet<&String>,
    type_to_module: &HashMap<String, String>,
    simplified_name: &str,
    defined_types: &HashSet<String>,
    enum_variants: &HashSet<String>,
) -> bool {
    known_types.contains(&type_name.to_string())
        && type_to_module[type_name] != simplified_name
        && !defined_types.contains(type_name)
        && !enum_variants.contains(type_name)
}

fn build_use_statements(
    referenced_types: &HashSet<String>,
    type_to_module: &HashMap<String, String>,
    defined_types: &HashSet<String>,
) -> String {
    let mut use_lines = Vec::new();

    for type_name in referenced_types {
        if !defined_types.contains(type_name) {
            if let Some(module) = type_to_module.get(type_name) {
                use_lines.push(format!(
                    "use crate::types::generated::generated::{}::{};\n",
                    module, type_name
                ));
            }
        }
    }

    use_lines.sort();
    use_lines.concat()
}

fn build_final_output(use_statements: &str, processed_content: &str) -> String {
    let mut final_output = String::new();
    final_output.push_str("/**\n");
    final_output.push_str(" * Copyright 2025-present Coinbase Global, Inc.\n");
    final_output.push_str(" *\n");
    final_output.push_str(
        " * Generated by Openapi Generator https://github.com/openapitools/openapi-generator\n",
    );
    final_output.push_str(" *\n");
    final_output.push_str(" * Licensed under the Apache License, Version 2.0 (the \"License\");\n");
    final_output.push_str(" * you may not use this file except in compliance with the License.\n");
    final_output.push_str(" * You may obtain a copy of the License at\n");
    final_output.push_str(" *\n");
    final_output.push_str(" *  http://www.apache.org/licenses/LICENSE-2.0\n");
    final_output.push_str(" *\n");
    final_output
        .push_str(" * Unless required by applicable law or agreed to in writing, software\n");
    final_output
        .push_str(" * distributed under the License is distributed on an \"AS IS\" BASIS,\n");
    final_output
        .push_str(" * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.\n");
    final_output
        .push_str(" * See the License for the specific language governing permissions and\n");
    final_output.push_str(" * limitations under the License.\n");
    final_output.push_str(" */\n\n");
    final_output.push_str("use serde::{Deserialize, Serialize};\n");

    if !use_statements.is_empty() {
        final_output.push_str(use_statements);
        final_output.push_str("\n");
    }

    final_output.push_str(processed_content);
    final_output
}

fn determine_module_name(
    override_defined: &Option<String>,
    processed_content: &str,
    simplified_name: &str,
    type_overrides: &HashMap<&str, &str>,
) -> String {
    override_defined.clone().unwrap_or_else(|| {
        let contains_override_definition = type_overrides.values().any(|override_name| {
            processed_content.contains(&format!("pub struct {}", override_name))
                || processed_content.contains(&format!("pub enum {}", override_name))
        });

        if contains_override_definition {
            if let Some(override_name) = type_overrides.values().find(|override_name| {
                processed_content.contains(&format!("pub struct {}", override_name))
                    || processed_content.contains(&format!("pub enum {}", override_name))
            }) {
                override_name.to_case(convert_case::Case::Snake)
            } else {
                simplified_name.to_string()
            }
        } else {
            simplified_name.to_string()
        }
    })
}

fn write_output_files(mod_rs_content: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Write mod.rs file
    let mod_rs_file = Path::new(GENERATED_DIR).join("mod.rs");
    fs::write(&mod_rs_file, mod_rs_content)?;

    // Create generated_types.rs that re-exports the generated module
    let generated_types_content = "/* Auto-generated types from OpenAPI specification\n\
                                  * DO NOT EDIT - This file is generated automatically\n\
                                  * Run 'make generate-types' to regenerate this file\n\
                                  */\n\n\
                                  pub mod generated;\n";

    let generated_types_file = Path::new("src").join("generated_types.rs");
    fs::write(&generated_types_file, generated_types_content)?;

    Ok(())
}

fn generate_fallback_types() {
    let generated_dir = Path::new(GENERATED_DIR);
    if !generated_dir.exists() {
        fs::create_dir_all(generated_dir).expect("Failed to create generated directory");
    }

    let fallback_content = "/* Fallback types - OpenAPI Generator CLI output not found\n\
                           * Please run 'make generate-types' to generate proper types\n\
                           */\n\
                           use serde::{Deserialize, Serialize};\n\n\
                           #[derive(Debug, Clone, Serialize, Deserialize)]\n\
                           pub struct ActivityMetadataOrders {\n\
                               // Empty struct for now - no properties defined in the API spec\n\
                           }\n";

    let fallback_file = generated_dir.join("fallback.rs");
    fs::write(&fallback_file, fallback_content).expect("Failed to write fallback types");

    let mod_rs_content = "/* Fallback generated module */\n\
                         pub mod fallback;\n";

    let mod_rs_file = generated_dir.join("mod.rs");
    fs::write(&mod_rs_file, mod_rs_content).expect("Failed to write mod.rs");

    let generated_types_content = "/* Fallback types - OpenAPI Generator CLI output not found */\n\
                                  pub mod generated;\n";

    let generated_types_file = Path::new("src").join("generated_types.rs");
    fs::write(&generated_types_file, generated_types_content)
        .expect("Failed to write generated_types.rs");
}

fn fix_custody_activity_type_references() -> Result<(), Box<dyn std::error::Error>> {
    let generated_dir = Path::new(GENERATED_DIR);
    if !generated_dir.exists() {
        eprintln!("Warning: Generated directory not found");
        return Ok(());
    }

    for entry in fs::read_dir(generated_dir)? {
        let entry = entry?;
        let path = entry.path();
        if let Some(extension) = path.extension() {
            if extension == "rs" {
                let filename = path.file_stem().unwrap().to_str().unwrap();
                // Only process custody-related files that might reference PrimeActivityType
                if filename.contains("address_book") {
                    let content = fs::read_to_string(&path)?;
                    // Simple string replacement to fix the type reference
                    let fixed_content = content.replace(
                        "prime_activity_type::PrimeActivityType",
                        "custody_activity_type::CustodyActivityType",
                    );
                    let fixed_content = fixed_content.replace(
                        "activity_type: PrimeActivityType",
                        "activity_type: CustodyActivityType",
                    );
                    if content != fixed_content {
                        fs::write(&path, fixed_content)?;
                        println!("Fixed PrimeActivityType references in {}", filename);
                    }
                }
            }
        }
    }

    Ok(())
}
