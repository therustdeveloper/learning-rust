crate::tui_module! {
    description: "Bit patterns of signed and unsigned integers",
    category: Advanced,
}

pub fn index(show: bool) {
    if show {
        let a: u16 = 50115;
        let b: i16 = -15421;

        println!("a: {:016b} {}", a, a);
        println!("b: {:016b} {}", b, b);
    }
}
