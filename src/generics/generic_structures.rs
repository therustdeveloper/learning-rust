//! src/generics/generic_structures.rs
/// Implementing Functionality
///
/// Generic Implementation
/// - Implements functionality for `any` type that can be used with the structure
///
/// Concrete Implementation
/// - Implements functionality for `only` the type specified
use crate::print_title;

trait Game {
    fn name(&self) -> String;
}

enum BoardGame {
    Chess,
    Monopoliy,
}

impl Game for BoardGame {
    fn name(&self) -> String {
        match self {
            BoardGame::Chess => "chess".to_string(),
            BoardGame::Monopoliy => "monopoly".to_string(),
        }
    }
}

enum VideoGame {
    PlayStation,
    Xbox,
}

impl Game for VideoGame {
    fn name(&self) -> String {
        match self {
            VideoGame::PlayStation => "play station".to_string(),
            VideoGame::Xbox => "xbox".to_string(),
        }
    }
}

struct PlayRoom<T: Game> {
    game: T,
}

impl<T: Game> PlayRoom<T> {
    pub fn game_info(&self) {
        println!("Game Name: {}", self.game.name());
    }
}

pub fn index(show: bool) {
    if show {
        print_title("Generic Structures");

        let mut video_room = PlayRoom {
            game: VideoGame::Xbox,
        };
        video_room.game_info();

        video_room = PlayRoom {
            game: VideoGame::PlayStation,
        };
        video_room.game_info();

        let mut board_room = PlayRoom {
            game: BoardGame::Monopoliy,
        };
        board_room.game_info();

        board_room = PlayRoom {
            game: BoardGame::Chess,
        };
        board_room.game_info();
    }
}
