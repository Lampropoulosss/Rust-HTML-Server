mod content_type;

use std::fs;
use std::io::prelude::*;
use std::net::TcpStream;

use content_type::content_type;

const BASE_PATH: &str = "./public";

pub fn handle_connection(method: &str, path: Option<String>, mut stream: TcpStream) {
    if method != "GET" {
        let response = "HTTP/1.1 405 Method Not Allowed\r\n\r\n";

        if let Err(err) = stream.write(response.as_bytes()) {
            eprintln!("Error: {}", err);
            return;
        }

        if let Err(err) = stream.flush() {
            eprintln!("Error: {}", err);
            return;
        }

        return;
    }

    if let Some(path) = path {
        let file_content = fs::read(&path);

        match file_content {
            Ok(content) => {
                let response = format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: {}\r\n\r\n",
                    content_type(path),
                );

                if let Err(err) = stream.write(response.as_bytes()) {
                    eprintln!("Error writing response: {}", err);
                    return;
                }

                if let Err(err) = stream.write_all(&content) {
                    eprintln!("Error writing response: {}", err);
                    return;
                }

                if let Err(err) = stream.flush() {
                    eprintln!("Error writing response: {}", err);
                    return;
                }
            }
            Err(_) => return404(stream),
        }
    } else {
        return404(stream)
    }
}

fn return404(mut stream: TcpStream) {
    let response: String;

    let file_contents = fs::read_to_string(format!("{}/404.html", BASE_PATH));

    match file_contents {
        Ok(content) => {
            response = format!(
                "HTTP/1.1 404 Not Found\r\nContent-Length: {}\r\n\r\n{}",
                content.len(),
                content
            );
        }
        Err(_) => {
            response = String::from("HTTP/1.1 404 Not Found\r\n\r\n");
        }
    }

    if let Err(err) = stream.write(response.as_bytes()) {
        eprintln!("Error writing response: {}", err);
        return;
    }

    if let Err(err) = stream.flush() {
        eprintln!("Error writing response: {}", err);
        return;
    }
}
