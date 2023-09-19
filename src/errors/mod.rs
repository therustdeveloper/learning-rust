mod closures;
mod error_propagation;

pub fn master(show: bool) {
    if show {
        println!("\n--- Errors using closures ---");
        closures::master();

        println!("\n--- Return Errors ---");
        error_propagation::master();
    }
}