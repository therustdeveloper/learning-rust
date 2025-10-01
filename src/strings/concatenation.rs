//! src/strings/concatenation.rs
pub fn index(show: bool) {
    if show {
        let s1 = String::from("Hello, ");
        let s2 = String::from("world!");
        // not s1 has been moved here and can no longer be used
        let s3 = s1 + &s2;

        println!("value of s3: {}", s3);

        concatenate_multiple_strings();
    }
}

fn concatenate_multiple_strings() {
    println!("--- concatenate multiple strings with format! macro  ---");
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");

    println!("value of s1: {}", s1);
    println!("value of s2: {}", s2);
    println!("value of s3: {}", s3);

    println!("value of s: {}", s);
}
