//! src/network/mod.rs
use crate::print_title;
use std::io::prelude::*;
use std::net::TcpListener;
use std::sync::{Arc, atomic::{AtomicBool, Ordering}, mpsc};
use std::{thread, time::Duration};

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

/// Run network examples with shutdown signal support for TUI mode
/// This version sends output through a channel and checks for shutdown signal
pub fn run_with_shutdown(tx: mpsc::Sender<String>, shutdown: Arc<AtomicBool>) {
    let send_msg = |msg: String| {
        let _ = tx.send(msg);
    };

    send_msg("Network Module Examples".to_string());
    send_msg("".to_string());

    // IP validation example
    let ip_address = "192.168.1.255";
    if is_valid_ip_with_channel(ip_address, &tx) {
        send_msg(format!("ip_address: {} is a valid IP address", ip_address));
    }

    send_msg("".to_string());

    // Check shutdown before starting server
    if shutdown.load(Ordering::Relaxed) {
        send_msg("Stopped by user".to_string());
        return;
    }

    // Starting the server
    match server_with_shutdown(&tx, &shutdown) {
        Ok(_) => send_msg("Server stopped normally".to_string()),
        Err(e) => send_msg(format!("Server error: {}", e)),
    }
}

fn is_valid_ip_with_channel(ip_address: &str, tx: &mpsc::Sender<String>) -> bool {
    let octets: Vec<&str> = ip_address.split(".").collect();

    if octets.len() != 4 {
        return false;
    }

    for octect in octets {
        match octect.parse::<u8>() {
            Ok(num) => {
                let _ = tx.send(format!("num: {}", num));
            }
            Err(_) => {
                return false;
            }
        }
    }

    true
}

fn server_with_shutdown(tx: &mpsc::Sender<String>, shutdown: &Arc<AtomicBool>) -> std::io::Result<()> {
    let listener = match TcpListener::bind("127.0.0.1:9090") {
        Ok(l) => l,
        Err(e) => {
            let _ = tx.send(format!("Failed to bind to 127.0.0.1:9090: {}", e));
            let _ = tx.send("The port might already be in use.".to_string());
            return Err(e);
        }
    };

    // Set non-blocking mode to allow shutdown checking
    listener.set_nonblocking(true)?;

    let _ = tx.send("Starting the server on port 9090".to_string());
    let _ = tx.send("Server will handle up to 5 connections or run for 60 seconds".to_string());
    let _ = tx.send("Press 's' or 'x' to stop the server".to_string());
    let _ = tx.send("Waiting for connections...".to_string());
    let _ = tx.send("".to_string());

    let start_time = std::time::Instant::now();
    let timeout = Duration::from_secs(60);
    let max_connections = 5;
    let mut connection_count = 0;

    while connection_count < max_connections && start_time.elapsed() < timeout {
        // Check shutdown signal
        if shutdown.load(Ordering::Relaxed) {
            let _ = tx.send("".to_string());
            let _ = tx.send("Shutdown signal received".to_string());
            break;
        }

        match listener.accept() {
            Ok((mut stream, addr)) => {
                connection_count += 1;
                let _ = tx.send(format!("[{}/{}] Connection received from: {}", connection_count, max_connections, addr));

                let mut buffer = [0; 1024];

                // Read with timeout
                stream.set_read_timeout(Some(Duration::from_secs(5)))?;

                loop {
                    // Check shutdown during message reading
                    if shutdown.load(Ordering::Relaxed) {
                        let _ = tx.send("Shutdown signal received while reading".to_string());
                        return Ok(());
                    }

                    match stream.read(&mut buffer) {
                        Ok(bytes_read) => {
                            if bytes_read == 0 {
                                let _ = tx.send(format!("  -> Connection from {} closed", addr));
                                break;
                            }

                            let message = String::from_utf8_lossy(&buffer[0..bytes_read]);
                            let _ = tx.send(format!("  -> Received message: {}", message.trim()));
                        }
                        Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock || e.kind() == std::io::ErrorKind::TimedOut => {
                            // Timeout or would block, continue
                            break;
                        }
                        Err(e) => {
                            let _ = tx.send(format!("  -> Error reading from stream: {}", e));
                            break;
                        }
                    }
                }

                let _ = tx.send("".to_string());
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                // No connection ready, sleep briefly and check again
                thread::sleep(Duration::from_millis(100));
            }
            Err(e) => {
                let _ = tx.send(format!("Connection error: {}", e));
            }
        }
    }

    let elapsed = start_time.elapsed();
    let _ = tx.send("".to_string());
    let _ = tx.send("Server statistics:".to_string());
    let _ = tx.send(format!("  - Handled {} connections", connection_count));
    let _ = tx.send(format!("  - Ran for {:.2} seconds", elapsed.as_secs_f64()));

    if connection_count == 0 {
        let _ = tx.send("".to_string());
        let _ = tx.send("No connections received. You can test the server by:".to_string());
        let _ = tx.send("  echo 'Hello' | nc 127.0.0.1 9090".to_string());
        let _ = tx.send("  Or: telnet 127.0.0.1 9090".to_string());
    }

    let _ = tx.send("".to_string());
    let _ = tx.send("Server shutting down.".to_string());

    Ok(())
}
