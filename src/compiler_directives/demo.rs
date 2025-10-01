//! src/compiler_directives/demo.rs
#![allow(unused_variables)] // notice the exclamation mark

pub fn index(show: bool) {
    if show {
        println!("-- Compiler Directives - File Scope --");

        let message = "Custom Message";

        show_message(false);
    }
}

fn show_message(show: bool) {
    if show {
        println!("-- Show Message Function --");

        let message = "Custom message in show_message function";
    }
}
