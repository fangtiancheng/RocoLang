use std::{env, fs, path::PathBuf};

fn main() {
    println!("cargo:rerun-if-changed=src/types.rs");
    println!("cargo:rerun-if-changed=src/types");
    println!("cargo:rerun-if-changed=src/stdlib");

    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let types_path = manifest_dir.join("src").join("types.rs");
    let types_dir = manifest_dir.join("src").join("types");
    let mut sources = vec![types_path];
    if types_dir.is_dir() {
        let mut domain_sources = fs::read_dir(types_dir)
            .unwrap()
            .map(|entry| entry.unwrap().path())
            .filter(|path| path.extension().is_some_and(|extension| extension == "rs"))
            .collect::<Vec<_>>();
        domain_sources.sort();
        sources.extend(domain_sources);
    }
    let type_names = sources
        .into_iter()
        .flat_map(|path| collect_public_type_names(&fs::read_to_string(path).unwrap()))
        .collect::<Vec<_>>();

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let output_path = out_dir.join("roco_type_list.rs");
    fs::write(output_path, render_type_list(&type_names)).unwrap();

    let stdlib_path = manifest_dir.join("src").join("stdlib");
    let stdlib_sources = rust_sources_recursive(&stdlib_path);
    let return_types = collect_stdlib_return_types(&stdlib_sources);
    let output_path = out_dir.join("roco_stdlib_return_types.rs");
    fs::write(output_path, render_stdlib_return_types(&return_types)).unwrap();
}

fn rust_sources_recursive(root: &PathBuf) -> Vec<PathBuf> {
    let mut sources = Vec::new();
    let mut pending = vec![root.clone()];
    while let Some(path) = pending.pop() {
        for entry in fs::read_dir(path).unwrap() {
            let path = entry.unwrap().path();
            if path.is_dir() {
                pending.push(path);
            } else if path.extension().is_some_and(|extension| extension == "rs") {
                sources.push(path);
            }
        }
    }
    sources.sort();
    sources
}

fn collect_stdlib_return_types(sources: &[PathBuf]) -> Vec<(String, String, String)> {
    let mut method_returns = std::collections::BTreeMap::new();
    for path in sources {
        let source = fs::read_to_string(path).unwrap();
        collect_trait_returns(&source, &mut method_returns);
    }

    let mut functions = std::collections::BTreeMap::new();
    for path in sources {
        let source = fs::read_to_string(path).unwrap();
        let module = path.file_stem().unwrap().to_string_lossy().to_string();
        collect_registration_returns(&source, &module, &method_returns, &mut functions);
    }
    functions
        .into_iter()
        .map(|((module, name), return_type)| (module, name, return_type))
        .collect()
}

fn collect_trait_returns(source: &str, methods: &mut std::collections::BTreeMap<String, String>) {
    let mut offset = 0;
    while let Some(relative) = source[offset..].find("fn ") {
        let start = offset + relative + 3;
        let name_end = source[start..]
            .find(|ch: char| !(ch.is_ascii_alphanumeric() || ch == '_'))
            .unwrap();
        let name = &source[start..start + name_end];
        let mut open = start + name_end;
        while source[open..]
            .chars()
            .next()
            .is_some_and(|character| character.is_whitespace())
        {
            open += 1;
        }
        let Some(close) = matching_delimiter(source, open, '(', ')') else {
            offset = open + 1;
            continue;
        };
        let Some(arrow) = source[close..].find("->") else {
            offset = close + 1;
            continue;
        };
        let return_start = close + arrow + 2;
        let Some(result_start) = source[return_start..].find("Result<") else {
            offset = return_start + 1;
            continue;
        };
        let type_start = return_start + result_start + "Result<".len();
        let Some(type_end) = matching_angle(source, type_start) else {
            offset = type_start + 1;
            continue;
        };
        methods.insert(
            name.to_string(),
            rust_type_to_rhai(&source[type_start..type_end]),
        );
        offset = type_end + 1;
    }
}

fn collect_registration_returns(
    source: &str,
    module: &str,
    method_returns: &std::collections::BTreeMap<String, String>,
    functions: &mut std::collections::BTreeMap<(String, String), String>,
) {
    let mut offset = 0;
    while let Some(relative) = source[offset..].find("register_stdlib_fn_") {
        let start = offset + relative;
        let Some(open_relative) = source[start..].find('(') else {
            break;
        };
        let open = start + open_relative;
        let Some(close) = matching_delimiter(source, open, '(', ')') else {
            offset = open + 1;
            continue;
        };
        let args = split_macro_args(&source[open + 1..close]);
        if args.len() >= 4 {
            let name = args[2].trim().trim_matches('"');
            let method = args[3].trim();
            if let Some(return_type) = method_returns.get(method) {
                functions.insert((module.to_string(), name.to_string()), return_type.clone());
            }
        }
        offset = close + 1;
    }
}

fn split_macro_args(source: &str) -> Vec<&str> {
    let mut result = Vec::new();
    let mut start = 0;
    let mut depth = 0;
    for (index, ch) in source.char_indices() {
        match ch {
            '(' | '[' | '{' | '<' => depth += 1,
            ')' | ']' | '}' | '>' => depth -= 1,
            ',' if depth == 0 => {
                result.push(&source[start..index]);
                start = index + 1;
            }
            _ => {}
        }
    }
    result.push(&source[start..]);
    result
}

fn matching_delimiter(source: &str, open: usize, left: char, right: char) -> Option<usize> {
    let mut depth = 0;
    for (index, ch) in source[open..].char_indices() {
        match ch {
            value if value == left => depth += 1,
            value if value == right => {
                depth -= 1;
                if depth == 0 {
                    return Some(open + index);
                }
            }
            _ => {}
        }
    }
    None
}

fn matching_angle(source: &str, open: usize) -> Option<usize> {
    matching_delimiter(source, open - 1, '<', '>')
}

fn rust_type_to_rhai(value: &str) -> String {
    let value = value.trim().replace(' ', "");
    if let Some(inner) = value
        .strip_prefix("Vec<")
        .and_then(|value| value.strip_suffix('>'))
    {
        return format!("{}[]", rust_type_to_rhai(inner));
    }
    match value.as_str() {
        "()" => "()".to_string(),
        "i64" | "u64" | "i32" | "u32" | "usize" => "int".to_string(),
        "bool" => "bool".to_string(),
        "String" | "ImmutableString" => "string".to_string(),
        "Dynamic" => "dynamic".to_string(),
        "Array" => "dynamic[]".to_string(),
        other => other.rsplit("::").next().unwrap_or(other).to_string(),
    }
}

fn render_stdlib_return_types(types: &[(String, String, String)]) -> String {
    let mut output = String::from(
        "// @generated by build.rs; do not edit.\n\
         pub(crate) fn generated_stdlib_return_type(module: &str, name: &str) -> Option<&'static str> {\n\
             match (module, name) {\n",
    );
    for (module, name, return_type) in types {
        output.push_str(&format!(
            "        ({module:?}, {name:?}) => Some({return_type:?}),\n"
        ));
    }
    output.push_str("        _ => None,\n    }\n}\n");
    output
}

fn collect_public_type_names(source: &str) -> Vec<String> {
    source
        .lines()
        .filter_map(|line| {
            let trimmed = line.trim_start();
            let rest = trimmed
                .strip_prefix("pub struct ")
                .or_else(|| trimmed.strip_prefix("pub enum "))?;
            let name = rest
                .split(|ch: char| !(ch.is_ascii_alphanumeric() || ch == '_'))
                .next()?;
            if name.is_empty() {
                None
            } else {
                Some(name.to_string())
            }
        })
        .collect()
}

fn render_type_list(type_names: &[String]) -> String {
    let mut output = String::from(
        "// @generated by build.rs; do not edit.\n\
         macro_rules! for_each_roco_type {\n\
         \x20   ($macro:ident) => {\n",
    );

    for name in type_names {
        output.push_str(&format!(
            "        $macro!(crate::types::{name}, \"{name}\");\n"
        ));
    }

    output.push_str("    };\n}\n");
    output
}
