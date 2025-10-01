//! src/typestate_pattern/activity.rs
use crate::print_title;

#[allow(unused)]
#[derive(Debug, Copy, Clone)]
struct LuggageId(usize);

#[derive(Debug)]
struct Luggage(LuggageId);

struct CheckIn(LuggageId);
struct OnLoad(LuggageId);
struct OffLoad(LuggageId);
struct AwaitingPickup(LuggageId);

#[allow(unused)]
struct EndCustody(LuggageId);

impl Luggage {
    fn new(id: LuggageId) -> Self {
        Luggage(id)
    }

    fn check_in(self) -> CheckIn {
        // self.0 because LuggageId is struct tuple
        CheckIn(self.0)
    }
}

impl CheckIn {
    fn onload(self) -> OnLoad {
        OnLoad(self.0)
    }
}

impl OnLoad {
    fn offload(self) -> OffLoad {
        OffLoad(self.0)
    }
}

impl OffLoad {
    fn carousel(self) -> AwaitingPickup {
        AwaitingPickup(self.0)
    }
}

impl AwaitingPickup {
    fn pickup(self) -> (Luggage, EndCustody) {
        (Luggage(self.0), EndCustody(self.0))
    }
}

pub fn index(show: bool) {
    if show {
        print_title("Activity Type State");

        let id = LuggageId(1);
        let luggage = Luggage::new(id);
        let luggage = luggage.check_in().onload().offload().carousel();
        let (luggage, _) = luggage.pickup();

        println!("Luggage: {:?}", luggage);
    }
}
