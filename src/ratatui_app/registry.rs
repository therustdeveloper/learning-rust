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

/// What happens when the user presses Enter on a module.
#[derive(Debug)]
pub enum Content {
    /// Run the module's `index(true)` as a whole.
    Run(fn()),
    /// Pick one of several sections and run it.
    Sections(&'static [Section]),
}

/// A learning module as shown in the dashboard.
#[derive(Debug)]
pub struct TuiModule {
    /// Value of `module_path!()` at the registration site, e.g. `common::arrays`.
    pub path: &'static str,
    pub description: &'static str,
    pub category: Category,
    pub content: Content,
}

impl TuiModule {
    /// The module directory name, e.g. `arrays`.
    pub fn name(&self) -> &'static str {
        self.path.rsplit("::").next().unwrap_or(self.path)
    }

    pub fn sections(&self) -> &'static [Section] {
        match self.content {
            Content::Sections(sections) => sections,
            Content::Run(_) => &[],
        }
    }
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
/// With sections, each entry names a child module whose `index(true)` runs on Enter:
///
/// ```ignore
/// crate::tui_module! {
///     description: "Fundamental Rust Arrays",
///     category: Basics,
///     sections: [
///         definition => "Definition",
///         predictability => "Predictability",
///     ],
/// }
/// ```
///
/// Without sections, the module's own `index(true)` runs on Enter:
///
/// ```ignore
/// crate::tui_module! {
///     description: "Hello world",
///     category: Basics,
/// }
/// ```
#[macro_export]
macro_rules! tui_module {
    (
        description: $description:literal,
        category: $category:ident,
        sections: [ $( $section:ident => $label:literal ),+ $(,)? ] $(,)?
    ) => {
        $crate::tui_module!(@submit $description, $category,
            $crate::ratatui_app::Content::Sections(&[
                $( $crate::ratatui_app::Section {
                    name: stringify!($section),
                    description: $label,
                    run: || $section::index(true),
                } ),+
            ])
        );
    };
    (
        description: $description:literal,
        category: $category:ident $(,)?
    ) => {
        $crate::tui_module!(@submit $description, $category,
            $crate::ratatui_app::Content::Run(|| index(true))
        );
    };
    (@submit $description:literal, $category:ident, $content:expr) => {
        ::inventory::submit! {
            $crate::ratatui_app::TuiModule {
                path: module_path!(),
                description: $description,
                category: $crate::ratatui_app::Category::$category,
                content: $content,
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn arrays_is_registered_with_its_sections() {
        let arrays = modules()
            .into_iter()
            .find(|module| module.name() == "arrays")
            .expect("arrays should self-register");

        let names: Vec<_> = arrays.sections().iter().map(|s| s.name).collect();
        assert_eq!(names, ["definition", "predictability", "arrays_with_enums"]);
        assert_eq!(arrays.category, Category::Basics);
    }
}
