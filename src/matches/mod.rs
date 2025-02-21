//! src/matches/mod.rs
mod catch_all;
mod definition;
mod if_let;
mod match_control_flow;
mod match_guard;
mod match_vector;
mod multiple_placeholders;
mod placeholder;
mod while_let;

pub fn master(show: bool) {
    if show {
        println!("\n-- Match Expression");

        definition::master(false);

        if_let::master(false);

        match_guard::master(false);

        catch_all::master(false);

        placeholder::master(false);

        multiple_placeholders::master(false);

        match_vector::master(false);

        while_let::master(false);

        match_control_flow::master(false);
    }
}
