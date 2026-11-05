pub fn index(show: bool) {
    if show {
        println!("-- Variable Scope --");

        let s1 = gives_ownership(); // gives_ownership moves its return value into s1
        println!("s1: {s1}");

        let s2 = String::from("hallo"); // s2 comes into scope

        let s3 = takes_and_gives_back(s2);
        println!("s3: {s3}");
    }
}

fn gives_ownership() -> String {
    String::from("yours")
}

fn takes_and_gives_back(a: String) -> String {
    // a comes into scope
    a // a is returned and moves out to the calling function
}
