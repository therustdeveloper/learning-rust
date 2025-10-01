//! src/errors/demo_custom_error.rs
use crate::print_title;
use chrono::{DateTime, Duration, Utc};
use thiserror::Error;

struct SubwayPass {
    id: usize,
    funds: isize,
    expires: DateTime<Utc>,
}

#[derive(Debug, Error)]
enum PassError {
    #[error("pass expired")]
    PassExpired,
    #[error("insufficient funds: {0}")]
    InsufficientFunds(isize),
    #[error("pass read error: {0}")]
    ReadError(String),
}

fn swipe_card(option: i8) -> Result<SubwayPass, PassError> {
    match option {
        1 => Ok(SubwayPass {
            id: 0,
            funds: 200,
            expires: Utc::now() + Duration::weeks(52),
        }),
        2 => Err(PassError::ReadError("Magstrip failed to read".to_owned())),
        3 => Ok(SubwayPass {
            id: 0,
            funds: 200,
            expires: Utc::now() - Duration::weeks(52),
        }),
        4 => Err(PassError::InsufficientFunds(3)),
        _ => Err(PassError::PassExpired),
    }
}

fn use_pass(pass: &mut SubwayPass, cost: isize) -> Result<(), PassError> {
    if Utc::now() > pass.expires {
        Err(PassError::PassExpired)
    } else if pass.funds - cost < 0 {
        Err(PassError::InsufficientFunds(pass.funds))
    } else {
        pass.funds -= cost;
        Ok(())
    }
}

pub fn index(show: bool) {
    if show {
        print_title("Demo Custom Error");

        let pass_status = swipe_card(4).and_then(|mut pass| {
            println!("Id: {}", pass.id);
            use_pass(&mut pass, 3)
        });
        match pass_status {
            Ok(_) => println!("ok to board"),
            Err(e) => match e {
                PassError::ReadError(s) => println!("Error reading: {}", s),
                PassError::PassExpired => println!("Error: pass has expired"),
                PassError::InsufficientFunds(f) => println!("Error: Insufficient funds {}", f),
            },
        }
    }
}
