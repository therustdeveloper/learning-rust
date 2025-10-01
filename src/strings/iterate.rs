//! src/strings/iterate.rs
pub fn index(show: bool) {
    if show {
        println!("--- Methods for Iterating  Over Strings ---");

        // Chars Methods
        chars_methods();

        // Bytes Method
        bytes_method();
    }
}

fn chars_methods() {
    println!("--- Chars Method ---");

    let s = "你好，世界!"; // hello, world! in Chinese (simplified)

    println!("s value: {}", s);
    println!("s length: {:?}", s.len());

    for c in s.chars() {
        println!("{c}");
    }
}

fn bytes_method() {
    println!("--- Bytes Method ---");

    let s = "你好，世界!"; // hello, world! in Chinese (simplified)

    for b in s.bytes() {
        println!("{b}"); // should print 16 in this case
    }
}
