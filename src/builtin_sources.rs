const COMBAT: &str = include_str!("../rocolib/combat.rhai");
const ROLE: &str = include_str!("../rocolib/role.rhai");
const SPIRIT: &str = include_str!("../rocolib/spirit.rhai");

pub fn source_paths() -> &'static [&'static str] {
    &["roco/combat", "roco/role", "roco/spirit"]
}

pub fn source_by_path(path: &str) -> Option<&'static str> {
    match path {
        "roco/combat" => Some(COMBAT),
        "roco/role" => Some(ROLE),
        "roco/spirit" => Some(SPIRIT),
        _ => None,
    }
}
