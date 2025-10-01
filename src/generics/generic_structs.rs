//! src/generics/generic_structs.rs
use crate::print_title;

struct Score<T> {
    user: String,
    team_score: T,
    scores: Vec<T>,
}

// Generic Methods
impl<T> Score<T> {
    fn last_score(&mut self) -> Option<&T> {
        self.scores.last()
    }
}

pub fn index(show: bool) {
    if show {
        print_title("Generic Structs");

        int_scores(false);

        flt_scores(true);
    }
}

fn int_scores(show: bool) {
    if show {
        print_title("Struct Int Scores");

        let mut int_score = Score {
            user: "Joe".to_string(),
            team_score: 77777,
            scores: vec![77, 777, 7777],
        };

        println!(
            "User: {}, Team Score: {}, Scores: {:?}",
            int_score.user, int_score.team_score, int_score.scores
        );

        println!("last score: {:?}", int_score.last_score());
    }
}

fn flt_scores(show: bool) {
    if show {
        print_title("Struct Float Scores");

        let mut flt_score = Score {
            user: "Doe".to_string(),
            team_score: 7777.7,
            scores: vec![7.0, 77.0, 777.0],
        };

        println!(
            "User: {}, Team Score: {}, Scores: {:?}",
            flt_score.user, flt_score.team_score, flt_score.scores
        );

        //println!("last score: {:?}", flt_score.last_score());

        //let last_score = flt_score.last_score();
        if let Some(last_score) = flt_score.last_score() {
            println!("last_score: {}", last_score);
        }
        /*let value: &f64 = match last_score {
            Some(last) => &last,
            None => &0.0,
        };
        println!("last score: {}", value);*/
    }
}
