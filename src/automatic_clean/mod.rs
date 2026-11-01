struct Resource {
    data: Vec<u8>,
}

crate::tui_module! {
    description: "Automatic resource cleanup when values go out of scope",
    category: Intermediate,
}

pub fn index(show: bool) {
    if show {
        let resource = Resource {
            data: vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
        };

        println!("{:?}", resource.data);
    }
}
