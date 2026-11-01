//! src/ratatui_app/registry.rs
//!
//! Self-registration for learning modules.
//!
//! A module opts into the dashboard by invoking [`tui_module!`](crate::tui_module)
//! from its own `mod.rs`. Registrations are collected at link time by the
//! `inventory` crate, so the dashboard never names a module explicitly: adding
//! or deleting a module directory is all it takes to show or hide it.

/// Groups modules in the details panel.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Category {
    Basics,
    Intermediate,
    Advanced,
    Concurrency,
    DataStructures,
    System,
}

impl Category {
    pub fn label(self) -> &'static str {
        match self {
            Category::Basics => "Basics",
            Category::Intermediate => "Intermediate",
            Category::Advanced => "Advanced",
            Category::Concurrency => "Concurrency",
            Category::DataStructures => "Data Structures",
            Category::System => "System",
        }
    }
}

/// A runnable lesson inside a module, e.g. `src/arrays/definition.rs`.
#[derive(Debug)]
pub struct Section {
    pub name: &'static str,
    pub description: &'static str,
    pub run: fn(),
}

/// A learning module as shown in the dashboard.
#[derive(Debug)]
pub struct TuiModule {
    /// Value of `module_path!()` at the registration site, e.g. `common::arrays`.
    pub path: &'static str,
    pub description: &'static str,
    pub category: Category,
    /// The module's own `index(true)`, run on Enter when it has no sections.
    /// `None` for `sections_only` modules, which have no `index` of their own.
    pub run: Option<fn()>,
}

impl TuiModule {
    /// The module directory name, e.g. `arrays`.
    pub fn name(&self) -> &'static str {
        self.path.rsplit("::").next().unwrap_or(self.path)
    }

    /// Child modules discovered by `build.rs`.
    pub fn sections(&self) -> &'static [Section] {
        discovered::sections(self.name())
    }
}

mod discovered {
    use super::Section;

    include!(concat!(env!("OUT_DIR"), "/tui_sections.rs"));
}

inventory::collect!(TuiModule);

/// Every registered module, sorted by name.
pub fn modules() -> Vec<&'static TuiModule> {
    let mut modules: Vec<_> = inventory::iter::<TuiModule>.into_iter().collect();
    modules.sort_by_key(|module| module.name());
    modules
}

/// Registers the enclosing module in the dashboard.
///
/// ```ignore
/// crate::tui_module! {
///     description: "Fundamental Rust Arrays",
///     category: Basics,
/// }
/// ```
///
/// Sections are discovered by `build.rs`: every `pub mod <name>;` whose file
/// defines `pub fn index(show: bool) {`. A module without sections runs its
/// own `index(true)` on Enter.
///
/// A module made only of sections passes `sections_only` and needs no `index`:
///
/// ```ignore
/// crate::tui_module! {
///     description: "String, &str and string functions",
///     category: Basics,
///     sections_only,
/// }
/// ```
#[macro_export]
macro_rules! tui_module {
    (description: $description:literal, category: $category:ident $(,)?) => {
        $crate::tui_module!(@submit $description, $category, Some(|| index(true)));
    };
    (description: $description:literal, category: $category:ident, sections_only $(,)?) => {
        $crate::tui_module!(@submit $description, $category, None);
    };
    (@submit $description:literal, $category:ident, $run:expr) => {
        ::inventory::submit! {
            $crate::ratatui_app::TuiModule {
                path: module_path!(),
                description: $description,
                category: $crate::ratatui_app::Category::$category,
                run: $run,
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn arrays_sections_are_discovered() {
        let arrays = modules()
            .into_iter()
            .find(|module| module.name() == "arrays")
            .expect("arrays should self-register");

        let names: Vec<_> = arrays.sections().iter().map(|s| s.name).collect();
        assert_eq!(names, ["arrays_with_enums", "definition", "predictability"]);
        assert_eq!(arrays.category, Category::Basics);
    }

    #[test]
    fn every_module_is_runnable() {
        for module in modules() {
            assert!(
                module.run.is_some() || !module.sections().is_empty(),
                "`{}` is sections_only but has no sections",
                module.name()
            );
        }
    }
}
