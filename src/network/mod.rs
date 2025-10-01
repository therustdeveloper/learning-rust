//! src/network/mod.rs
use crate::print_title;
use std::io::prelude::*;
use std::net::TcpListener;

pub fn index(show: bool) {
    if show {
        print_title("Network");

        let ip_address = "192.168.1.255";
        if is_valid_ip(ip_address) {
            println!("ip_address: {} is a valid IP address", ip_address);
        }

        // Starting the server
        server().unwrap();
    }
}

fn is_valid_ip(ip_address: &str) -> bool {
    let octets: Vec<&str> = ip_address.split(".").collect();

    if octets.len() != 4 {
        return false;
    }

    for octect in octets {
        match octect.parse::<u8>() {
            Ok(num) => {
                println!("num: {}", num);
            }
            Err(_) => {
                return false;
            }
        }
    }

    true
}

fn server() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:9090")?;

    println!("Starting the server in port 9090");

    for stream in listener.incoming() {
        let mut stream = stream?;

        let mut buffer = [0; 1024];

        loop {
            let bytes_read = stream.read(&mut buffer)?;

            if bytes_read == 0 {
                break;
            }

            let message = String::from_utf8_lossy(&buffer[0..bytes_read]);

            println!("Received message: {}", message);
        }
    }

    Ok(())
}
