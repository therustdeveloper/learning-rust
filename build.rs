//! build.rs
//!
//! Discovers TUI sections for `crate::tui_module!`.
//!
//! For every `src/<module>/mod.rs` that invokes `crate::tui_module!`, each
//! `pub mod <name>;` whose file defines `pub fn index(show: bool) {` becomes a
//! section. Everything is written to a single `$OUT_DIR/tui_sections.rs`,
//! included once by `src/ratatui_app/registry.rs`.

use std::{env, fs, path::Path};

const INDEX_SIGNATURE: &str = "pub fn index(show: bool) {";
const REGISTRATION: &str = "crate::tui_module!";

fn main() {
    println!("cargo:rerun-if-changed=src");

    let mut modules: Vec<_> = fs::read_dir("src")
        .expect("src/ is readable")
        .flatten()
        .map(|entry| entry.path())
        .filter_map(|dir| {
            let mod_rs = fs::read_to_string(dir.join("mod.rs")).ok()?;
            let name = dir.file_name()?.to_str()?.to_string();
            registers(&mod_rs).then(|| (name, sections(&dir, &mod_rs)))
        })
        .collect();
    modules.sort();

    let arms: String = modules
        .iter()
        .map(|(module, sections)| format!("        {module:?} => &[\n{sections}        ],\n"))
        .collect();
    let generated = format!(
        "/// Sections discovered by build.rs for each registered module.\n\
         pub fn sections(module: &str) -> &'static [Section] {{\n    \
             match module {{\n{arms}        _ => &[],\n    }}\n}}\n"
    );

    let out_dir = env::var("OUT_DIR").expect("OUT_DIR is set by cargo");
    fs::write(Path::new(&out_dir).join("tui_sections.rs"), generated).unwrap();
}

/// Whether `mod.rs` registers itself with the dashboard.
fn registers(mod_rs: &str) -> bool {
    mod_rs
        .lines()
        .any(|line| line.trim_start().starts_with(REGISTRATION))
}

/// `Section` literals for every child module following the `index` pattern.
fn sections(module_dir: &Path, mod_rs: &str) -> String {
    let module = module_dir.file_name().unwrap().to_str().unwrap();
    declared_modules(mod_rs)
        .filter(|name| has_index(module_dir, name))
        .map(|name| {
            format!(
                "            Section {{ name: {name:?}, description: {:?}, run: || crate::{module}::{name}::index(true) }},\n",
                title_case(name)
            )
        })
        .collect()
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
