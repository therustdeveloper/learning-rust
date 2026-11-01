#![allow(unnecessary_transmutes)]
crate::tui_module! {
    description: "Interpreting float bits with transmute",
    category: Advanced,
}

pub fn index(show: bool) {
    if show {
        let a: f32 = 42.42;
        let frankentype: u32 = unsafe { std::mem::transmute(a) };

        println!("{}", frankentype);
        println!("{:032b}", frankentype);

        let b: f32 = unsafe { std::mem::transmute(frankentype) };
        println!("{}", b);

        assert_eq!(a, b);
    }
}
