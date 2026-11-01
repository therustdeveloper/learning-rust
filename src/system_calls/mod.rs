//! src/system_calls/mod.rs
use crate::print_title;
pub mod c_call;
pub mod raw_syscall;

crate::tui_module! {
    description: "Making system calls",
    category: System,
}

pub fn index(show: bool) {
    if show {
        print_title("System Calls");

        raw_syscall::index(false);

        c_call::index(false);
    }
}
