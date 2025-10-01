//! src/matches/mod.rs
pub mod catch_all;
pub mod definition;
pub mod if_let;
pub mod match_control_flow;
pub mod match_guard;
pub mod match_vector;
pub mod multiple_placeholders;
pub mod pattern_match;
pub mod placeholder;
pub mod while_let;

pub fn index(show: bool) {
    if show {
        println!("\n-- Match Expression");

        definition::index(false);

        if_let::index(false);

        match_guard::index(false);

        catch_all::index(false);

        placeholder::index(false);

        multiple_placeholders::index(false);

        match_vector::index(false);

        while_let::index(false);

        match_control_flow::index(false);

        pattern_match::index(false);
    }
}
