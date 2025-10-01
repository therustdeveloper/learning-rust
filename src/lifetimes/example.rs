//! src/lifetimes/example.rs
#[derive(Debug)]
enum Part {
    Bolt,
    Panel,
}

#[derive(Debug)]
struct RobotArm<'a> {
    part: &'a Part,
}

#[derive(Debug)]
struct AssemblyLine {
    parts: Vec<Part>,
}

pub fn index(show: bool) {
    if show {
        println!("\n--- Lifetime Example");

        let line = AssemblyLine {
            parts: vec![Part::Bolt, Part::Panel],
        };
        {
            let arm = RobotArm {
                part: &line.parts[0],
            };
            println!("Arm: {:?}", arm);
            println!("Arm Part: {:?}", arm.part);
        }

        println!("Line: {:?}", line);
    }
}
