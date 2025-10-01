//! src/testing/demo.rs
pub fn index(show: bool) {
    if show {
        println!("\n--- Testing Demo");

        let msg = all_caps("nasa");
        println!("msg: {}", msg);
    }
}

fn all_caps(word: &str) -> String {
    word.to_uppercase()
}

#[cfg(test)]
pub mod test {
    use crate::testing::demo::all_caps;

    #[test]
    fn check_all_caps() {
        let result = all_caps("hallo");
        let expected = String::from("HALLO");
        assert_eq!(result, expected, "string should be all uppercase");
    }
}
