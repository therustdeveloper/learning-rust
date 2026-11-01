//! src/main.rs
fn main() {
    local_function(false);
    common::automatic_clean::index(false);
    common::bit_patterns::index(false);
    common::cubesats::index(false);
    common::deconstructing_float::index(false);
    common::formatted_print::index(false);
    common::hello_world::index(false);
    common::inspecting_endianness::index(false);
    common::interpreting_float::index(false);
    common::machine_learning::index(false);
    common::mandelbrot::index(false);
    common::network::index(false);
    common::numbers::index(false);
    common::open_file::index(false);
    common::ratatui_app::index(true);
    common::zero_cost_abstraction::index(false);
}

fn local_function(show: bool) {
    if show {
        println!("--- Local Function ---");
        println!("Hello from local function!");
    }
}
