//! src/system_calls/mod.rs
pub mod c_call;
pub mod raw_syscall;

crate::tui_module! {
    description: "Making system calls",
    category: System,
    sections_only,
}
