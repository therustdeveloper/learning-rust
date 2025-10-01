//! src/exercises/ex03/fah_to_cel.rs
pub fn index() {
    println!("--- Convert Fahrenheit-Celsius ---");

    let f = 26.0;
    let c = convert_to_celsius(f);
    println!("{:?} in Celsius is : {:?}", f, c);
}

fn convert_to_celsius(f: f32) -> f32 {
    (5_f32 / 9_f32) * (f - 32.0)
}
