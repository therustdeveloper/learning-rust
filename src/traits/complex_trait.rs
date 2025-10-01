//! src/traits/complex_trait.rs
use std::fmt::Debug;

#[derive(Debug)]
struct Monster {
    health: i32,
}

#[derive(Debug)]
struct Wizard {
    health: i32,
}

#[derive(Debug)]
struct Ranger {
    health: i32,
}

trait DisplayHealth {
    fn health(&self) -> i32;
}

trait MonsterBehavior: Debug {
    fn take_damage(&mut self, damage: i32);
    fn display_self(&self) {}
}

trait Magic {}

impl MonsterBehavior for Monster {
    fn take_damage(&mut self, damage: i32) {
        self.health -= damage;
    }
}

impl DisplayHealth for Monster {
    fn health(&self) -> i32 {
        self.health
    }
}

trait FightClose {
    fn attack_with_sword<T: MonsterBehavior>(&self, opponent: &mut T) {
        println!("You attack with your sword!");
        opponent.take_damage(10);
        opponent.display_self();
    }

    fn attack_with_hand<T: MonsterBehavior>(&self, opponent: &mut T) {
        println!("You attack with your hand!");
        opponent.take_damage(2);
        opponent.display_self();
    }
}

impl FightClose for Wizard {}
impl FightClose for Ranger {}
impl Magic for Wizard {}

trait FightFromDistance: Debug {
    fn attack_with_bow(&self, opponent: &mut Monster, distance: u32) {
        if distance < 10 {
            opponent.health -= 10;
            println!(
                "Bow attack! Opponent's health {}. You are now at: {:?}",
                opponent.health, self
            )
        }
    }

    fn attack_with_rock(&self, opponent: &mut Monster, distance: u32) {
        if distance < 3 {
            opponent.health -= 4;
        }
        println!(
            "Rock attack! Opponent's health {}. You are now at: {:?}",
            opponent.health, self
        );
    }
}

impl FightFromDistance for Ranger {}

fn fireball<T>(pc: &T, opponent: &mut Monster, distance: u32)
where
    T: Magic + Debug,
{
    if distance < 15 {
        opponent.health -= 20;
        println!(
            "A massive fireball! Opponent's health: {}. You are now at: {pc:?}",
            opponent.health
        );
    }
}

pub fn index(show: bool) {
    if show {
        println!("--- Complex Trait");

        let radagast = Wizard { health: 70 };
        let aragorn = Ranger { health: 80 };
        let mut uruk_hai = Monster { health: 40 };

        println!("Initial Health");
        println!("Radagast's Health: {:?}", radagast.health);
        println!("Aragorn's Health: {:?}", aragorn.health);
        println!("Uruk Hai's Health: {:?}", uruk_hai.health);

        radagast.attack_with_sword(&mut uruk_hai);
        radagast.attack_with_hand(&mut uruk_hai);
        aragorn.attack_with_bow(&mut uruk_hai, 7);
        aragorn.attack_with_rock(&mut uruk_hai, 7);

        println!("Final Health");
        println!("Radagast's Health: {:?}", radagast.health);
        println!("Aragorn's Health: {:?}", aragorn.health);
        println!("Uruk Hai's Health: {:?}", uruk_hai.health());

        fireball(&radagast, &mut uruk_hai, 8);
    }
}
