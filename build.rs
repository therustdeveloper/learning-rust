//! build.rs
//!
//! Discovers TUI sections for `crate::tui_module!`.
//!
//! For every `src/<module>/mod.rs`, each `pub mod <name>;` whose file defines
//! `pub fn index(show: bool) {` becomes a section. The result is written to
//! `$OUT_DIR/tui_sections/src/<module>/mod.rs` as a `&[Section]` expression,
//! which the macro pulls in with `include!`.

use std::{env, fs, path::Path};

const INDEX_SIGNATURE: &str = "pub fn index(show: bool) {";

fn main() {
    println!("cargo:rerun-if-changed=src");

    let out_dir = env::var("OUT_DIR").expect("OUT_DIR is set by cargo");
    let src = Path::new("src");

    for entry in fs::read_dir(src).expect("src/ is readable").flatten() {
        let module_dir = entry.path();
        let Ok(mod_rs) = fs::read_to_string(module_dir.join("mod.rs")) else {
            continue;
        };

        let sections: String = declared_modules(&mod_rs)
            .filter(|name| has_index(&module_dir, name))
            .map(|name| {
                format!(
                    "    crate::ratatui_app::Section {{ name: {name:?}, description: {:?}, run: || {name}::index(true) }},\n",
                    title_case(name)
                )
            })
            .collect();

        let target = Path::new(&out_dir)
            .join("tui_sections")
            .join(&module_dir)
            .join("mod.rs");
        fs::create_dir_all(target.parent().unwrap()).unwrap();
        fs::write(target, format!("&[\n{sections}]\n")).unwrap();
    }
}

/// Names from `pub mod <name>;` lines, in declaration order.
fn declared_modules(mod_rs: &str) -> impl Iterator<Item = &str> {
    mod_rs.lines().filter_map(|line| {
        line.trim()
            .strip_prefix("pub mod ")?
            .strip_suffix(';')
            .map(str::trim)
    })
}

/// Whether `<name>.rs` or `<name>/mod.rs` follows the `index(show: bool)` pattern.
fn has_index(module_dir: &Path, name: &str) -> bool {
    [
        module_dir.join(format!("{name}.rs")),
        module_dir.join(name).join("mod.rs"),
    ]
    .iter()
    .filter_map(|path| fs::read_to_string(path).ok())
    .any(|source| source.lines().any(|line| line.trim() == INDEX_SIGNATURE))
}

/// `match_statements` -> `Match Statements`
fn title_case(name: &str) -> String {
    name.split('_')
        .map(|word| {
            let mut chars = word.chars();
            chars.next().map_or(String::new(), |first| {
                first.to_uppercase().chain(chars).collect()
            })
        })
        .collect::<Vec<_>>()
        .join(" ")
}
