//! src/typestate_pattern/demo.rs
use crate::print_title;

// State is a Generic Parameter
struct Employee<State> {
    name: String,
    state: State,
}

impl<State> Employee<State> {
    fn transition<NextState>(self, state: NextState) -> Employee<NextState> {
        Employee {
            name: self.name,
            state,
        }
    }
}

struct Agreement;
struct Signature;
struct Training;
struct FailedTraining {
    score: u8,
}
struct OnboardingCompleted {
    score: u8,
}

impl Employee<Agreement> {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            state: Agreement,
        }
    }

    fn read_agreement(self) -> Employee<Signature> {
        self.transition(Signature)
    }
}

impl Employee<Signature> {
    fn sign(self) -> Employee<Training> {
        self.transition(Training)
    }
}

impl Employee<Training> {
    fn train(self, score: u8) -> Result<Employee<OnboardingCompleted>, Employee<FailedTraining>> {
        if score >= 7 {
            Ok(self.transition(OnboardingCompleted { score }))
        } else {
            Err(self.transition(FailedTraining { score }))
        }
    }
}

pub fn index(show: bool) {
    if show {
        print_title("TypeState Pattern Demo");

        let employee = Employee::new("Sara");
        let onboarded = employee.read_agreement().sign().train(7);
        match onboarded {
            Ok(complete) => println!("Onboarding completed, score: {}", complete.state.score),
            Err(failed) => println!("Training failed, score: {}", failed.state.score),
        }
    }
}
