//! src/guessing_game/mod.rs
pub mod guessing;

pub fn index(show: bool) {
    if show {
        guessing::index(false);
    }
}
