//! src//home/william/workspace/rust/learning-rust/src/main.rs
fn main() {
    local_function(false);
    common::hello_world::master(false);
    common::guessing_game::master(false);
    common::basics::master(false);
    common::ownership::master(true);
    common::structs::master(false);
    common::methods::master(false);
    common::enums::master(false);
    common::random_numbers::master(false);
    common::algorithms::master(false);
    common::collections::master(false);
    common::hashmaps::master(false);
    common::exercises::master(false);
    common::errors::master(false);
    common::traits::master(false);
    common::lifetimes::master(false);
    common::strings::master(false);
    common::generics::master(false);
    common::testing::master(false);
    common::closures::master(false);
    common::iterators::master(false);
    common::doc::master(false);
    common::boxes::master(false);
    common::custom_smart_pointer::master(false);
    common::rc_t::master(false);
    common::messenger::master(false);
    common::tree_data_structure::master(false);
    common::threads::master(false);
    common::joinhandle::master(false);
    common::message_passing::master(false);
    common::api_mutex::master(false);
    common::object_oriented::master(false);
    common::draw::master(false);
    common::post::master(false);
    common::patterns::master(false);
    common::destructuring_structs::master(false);
    common::unsafe_code::master(false);
    common::extern_functions::master(false);
    common::static_variable::master(false);
    common::unsafe_trait::master(false);
    common::operator_overloading::master(false);
    common::add_millimeters_to_meters::master(false);
    common::vectors::master(false);
    common::futures::master(false);
    common::get_type::master(false);
    common::mutable_reference::master(false);
    common::decisions::master(false);
    common::matches::master(false);
    common::loops::master(false);
    common::while_loop::master(false);
    common::tuples::master(false);
    common::expressions::master(false);
    common::arrays::master(false);
    common::print_var_type::master(false);
    common::move_vars::master(false);
    common::data_collections::master(false);
    common::references::master(false);
    common::advanced_match::master(false);
    common::optionals::master(false);
    common::results::master(false);
    common::ranges::master(false);
    common::modules::master(false);
    common::external_crates::master(false);
    common::user_input::master(false);
    common::webserver::master(false);
    common::new_type::master(false);
    common::typestate_pattern::master(false);
    common::for_loops::master(false);
    common::slices::master(false);
    common::type_aliases::master(false);
    common::numbers::master(false);
    common::mandelbrot::master(false);
    common::network::master(false);
    common::cubesats::master(false);
    common::inspecting_endianness::master(false);
    common::system_calls::master(false);
    common::shadowing::master(false);
    common::machine_learning::master(false);
    common::channels::master(false);
    common::macros::master(false);
    common::rust_atomics_and_locks::master(false);
    common::interior_mutability::master(false);
    common::cow_type::master(false);
    common::functions::master(false);
    common::concurrency::master(false);
    common::variables_and_mutability::master(false);
    common::compiler_directives::master(false);
}

fn local_function(show: bool) {
    if show {
        println!("--- Local Function ---");
        println!("Hello from local function!");
    }
}
