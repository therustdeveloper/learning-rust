//! src/custom_smart_pointer/definition.rs
pub fn index(show: bool) {
    if show {
        definition();
    }
}

fn definition() {
    println!("\n-- Definition");

    let _c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let _d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created. (Called before end of main)");
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!(
            "Dropping CustomSmartPointer with data `{}`!. (Called at the end of main)",
            self.data
        );
    }
}
