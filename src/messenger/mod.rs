//! src/messenger/mod.rs
pub mod definition;
pub mod mock_messenger;

crate::tui_module! {
    description: "Mock objects with RefCell: a messenger app",
    category: Advanced,
    sections_only,
}
