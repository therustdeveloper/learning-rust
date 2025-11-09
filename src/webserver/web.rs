//! src/webserver/web.rs
use crate::print_title;
use crate::ThreadPool;

use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc, Arc,
    },
    thread,
    time::{Duration, Instant},
};

pub fn index(show: bool) {
    if show {
        print_title("Webserver");

        let listener = match TcpListener::bind("127.0.0.1:7777") {
            Ok(l) => l,
            Err(e) => {
                println!("Failed to bind to 127.0.0.1:7777: {}", e);
                println!("The port might already be in use.");
                return;
            }
        };

        // Set non-blocking mode to allow timeout checking
        listener.set_nonblocking(true).unwrap();

        let pool = ThreadPool::new(4);
        let running = Arc::new(AtomicBool::new(true));

        println!("Web server listening on http://127.0.0.1:7777");
        println!("Server will handle up to 4 requests or run for 30 seconds");
        println!("Press Ctrl+C to stop (not supported in TUI mode)");
        println!("Waiting for connections...\n");

        let start_time = Instant::now();
        let timeout = Duration::from_secs(30);
        let max_connections = 4;
        let mut count = 0;

        while count < max_connections && start_time.elapsed() < timeout {
            match listener.accept() {
                Ok((stream, addr)) => {
                    count += 1;
                    println!(
                        "[{}/{}] Connection received from: {}",
                        count, max_connections, addr
                    );

                    pool.execute(move || {
                        handle_connection(stream);
                    });
                }
                Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    // No connection ready, sleep briefly and check again
                    thread::sleep(Duration::from_millis(100));
                }
                Err(e) => {
                    eprintln!("Connection error: {}", e);
                }
            }

            // Check if we should continue
            if !running.load(Ordering::Relaxed) {
                break;
            }
        }

        let elapsed = start_time.elapsed();
        println!("\nServer statistics:");
        println!("  - Handled {} connections", count);
        println!("  - Ran for {:.2} seconds", elapsed.as_secs_f64());

        if count == 0 {
            println!("\nNo connections received. You can test the server by:");
            println!("  curl http://127.0.0.1:7777");
            println!("  Or open http://127.0.0.1:7777 in your browser");
        }

        println!("\nServer shutting down.");
    }
}

fn handle_connection(mut stream: TcpStream) {
    let peer_addr = stream.peer_addr().unwrap();
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    println!("  Request from {}: {}", peer_addr, request_line);

    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => {
            println!("  -> Serving homepage");
            ("HTTP/1.1 200 OK", "hello.html")
        }
        "GET /sleep HTTP/1.1" => {
            println!("  -> Simulating slow request (7s delay)...");
            thread::sleep(Duration::from_secs(7));
            println!("  -> Slow request completed");
            ("HTTP/1.1 200 OK", "hello.html")
        }
        _ => {
            println!("  -> Route not found, returning 404");
            ("HTTP/1.1 404 NOT FOUND", "404.html")
        }
    };

    let contents = match fs::read_to_string(filename) {
        Ok(c) => c,
        Err(_) => {
            println!("  -> File {} not found, using placeholder", filename);
            format!(
                "<html><body><h1>{}</h1><p>File: {}</p></body></html>",
                if filename == "404.html" {
                    "404 Not Found"
                } else {
                    "Hello!"
                },
                filename
            )
        }
    };

    let length = contents.len();

    let response = format!(
        "{status_line}\r\n\
        Content-Length: {length}\r\n\r\n\
        {contents}"
    );

    stream.write_all(response.as_bytes()).unwrap();
    println!("  -> Response sent to {}", peer_addr);
}

/// Run the webserver with shutdown signal support for TUI mode
/// This version sends output through a channel instead of printing directly
pub fn run_with_shutdown(tx: mpsc::Sender<String>, shutdown: Arc<AtomicBool>) {
    let send_msg = |msg: String| {
        let _ = tx.send(msg);
    };

    let listener = match TcpListener::bind("127.0.0.1:7777") {
        Ok(l) => l,
        Err(e) => {
            send_msg(format!("Failed to bind to 127.0.0.1:7777: {}", e));
            send_msg("The port might already be in use.".to_string());
            return;
        }
    };

    // Set non-blocking mode to allow shutdown checking
    listener.set_nonblocking(true).unwrap();

    let pool = ThreadPool::new(4);

    send_msg("Web server listening on http://127.0.0.1:7777".to_string());
    send_msg("Server will handle up to 4 requests or run for 30 seconds".to_string());
    send_msg("Press 's' or 'x' to stop the server".to_string());
    send_msg("Waiting for connections...\n".to_string());

    let start_time = Instant::now();
    let timeout = Duration::from_secs(30);
    let max_connections = 4;
    let mut count = 0;

    while count < max_connections && start_time.elapsed() < timeout {
        // Check shutdown signal
        if shutdown.load(Ordering::Relaxed) {
            send_msg("\nShutdown signal received".to_string());
            break;
        }

        match listener.accept() {
            Ok((stream, addr)) => {
                count += 1;
                let msg = format!(
                    "[{}/{}] Connection received from: {}",
                    count, max_connections, addr
                );
                send_msg(msg);

                // Clone tx for the thread
                let tx_clone = tx.clone();
                pool.execute(move || {
                    handle_connection_with_channel(stream, tx_clone);
                });
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                // No connection ready, sleep briefly and check again
                thread::sleep(Duration::from_millis(100));
            }
            Err(e) => {
                send_msg(format!("Connection error: {}", e));
            }
        }
    }

    let elapsed = start_time.elapsed();
    send_msg("\nServer statistics:".to_string());
    send_msg(format!("  - Handled {} connections", count));
    send_msg(format!("  - Ran for {:.2} seconds", elapsed.as_secs_f64()));

    if count == 0 {
        send_msg("\nNo connections received. You can test the server by:".to_string());
        send_msg("  curl http://127.0.0.1:7777".to_string());
        send_msg("  Or open http://127.0.0.1:7777 in your browser".to_string());
    }

    send_msg("\nServer shutting down.".to_string());
}

/// Handle connection and send output to channel instead of stdout
fn handle_connection_with_channel(mut stream: TcpStream, tx: mpsc::Sender<String>) {
    let send_msg = |msg: String| {
        let _ = tx.send(msg);
    };

    let peer_addr = stream.peer_addr().unwrap();
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    send_msg(format!("  Request from {}: {}", peer_addr, request_line));

    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => {
            send_msg("  -> Serving homepage".to_string());
            ("HTTP/1.1 200 OK", "hello.html")
        }
        "GET /sleep HTTP/1.1" => {
            send_msg("  -> Simulating slow request (7s delay)...".to_string());
            thread::sleep(Duration::from_secs(7));
            send_msg("  -> Slow request completed".to_string());
            ("HTTP/1.1 200 OK", "hello.html")
        }
        _ => {
            send_msg("  -> Route not found, returning 404".to_string());
            ("HTTP/1.1 404 NOT FOUND", "404.html")
        }
    };

    let contents = match fs::read_to_string(filename) {
        Ok(c) => c,
        Err(_) => {
            send_msg(format!(
                "  -> File {} not found, using placeholder",
                filename
            ));
            format!(
                "<html><body><h1>{}</h1><p>File: {}</p></body></html>",
                if filename == "404.html" {
                    "404 Not Found"
                } else {
                    "Hello!"
                },
                filename
            )
        }
    };

    let length = contents.len();

    let response = format!(
        "{status_line}\r\n\
        Content-Length: {length}\r\n\r\n\
        {contents}"
    );

    stream.write_all(response.as_bytes()).unwrap();
    send_msg(format!("  -> Response sent to {}", peer_addr));
}
