mod handle_connection;
mod sanitize_path;

use std::net::TcpListener;

use handle_connection::handle_connection;
use sanitize_path::sanitize_path;
use HTML_Server_Rust::ThreadPool;

use std::io::prelude::*;

const MAX_THREADS: usize = 4;
const BASE_PATH: &str = "./public";

fn main() {
    let listener = TcpListener::bind("0.0.0.0:80").unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    });

    let thread_pool = ThreadPool::new(MAX_THREADS);

    loop {
        let (mut stream, addr) = match listener.accept() {
            Ok((stream, addr)) => (stream, addr),
            Err(err) => {
                println!("Could not accept connection.\nError: {}", err);
                break;
            }
        };

        println!("Connection established with {}", addr);

        thread_pool.execute(|| {
            let mut buffer = [0; 1024];

            if let Err(err) = stream.read(&mut buffer) {
                println!("Could not read request to buffer.\nError: {}", err);
                return;
            }

            let first_line = match buffer.lines().next() {
                Some(Ok(line)) => line,
                Some(Err(err)) => {
                    eprintln!("Error parsing the line: {}", err);
                    return;
                }
                None => {
                    eprintln!("Error: No data received.");
                    return;
                }
            };

            let parts: Vec<&str> = first_line.split_whitespace().collect();

            println!("Request: {}", first_line);

            if parts.len() < 3 {
                return; // Invalid Request
            }

            let sanitized_path = sanitize_path(parts[1], BASE_PATH);

            handle_connection(parts[0], sanitized_path, stream);
        });
    }
}
