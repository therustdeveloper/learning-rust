//! src/guessing_game/mod.rs
pub mod guessing;

crate::tui_module! {
    description: "The classic guessing game",
    category: Basics,
}

pub fn index(show: bool) {
    if show {
        guessing::index(false);
    }
}
