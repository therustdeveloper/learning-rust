//! src/strings/string_is_encoded.rs
pub fn index(show: bool) {
    if show {
        println!("--- &str and String are encoded with UTF-8");

        let name = "アドリアン・ファーレンハイツ・ツェペシュ";

        let other_name = String::from("アルカード");

        println!("name: {:?}", name);
        println!("other_name: {:?}", other_name);
    }
}
