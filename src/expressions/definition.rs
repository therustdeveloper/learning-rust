//! src/expressions/definition.rs
enum Access {
    Admin,
    Manager,
    User,
    Guest,
}

fn validate_permission(access: Access) -> bool {
    match access {
        Access::Admin => true,
        Access::Manager => true,
        Access::User => false,
        Access::Guest => false,
    }
}

fn print_access_level(can_access_file: bool) {
    if can_access_file {
        println!("Access allowed!");
    } else {
        println!("Access denied!");
    }
}

pub fn index(show: bool) {
    if show {
        println!("\n--- Definition");

        // secret file: admins & maganer only
        let mut access_level = Access::Guest;
        let mut can_access_file = validate_permission(access_level);
        print_access_level(can_access_file);

        access_level = Access::Manager;
        can_access_file = validate_permission(access_level);
        print_access_level(can_access_file);

        access_level = Access::Admin;
        can_access_file = validate_permission(access_level);
        print_access_level(can_access_file);

        access_level = Access::User;
        can_access_file = validate_permission(access_level);
        print_access_level(can_access_file);
    }
}
