use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::fs::File;
use std::thread;

mod utils;

pub struct HTTPResponse {
    version: String,
    status: String,
    content_type: String,
    content: String,
}

impl HTTPResponse {
    pub fn new() -> Self {
        Self {
            version: String::from("HTTP/1.1"),
            status: String::from("200 OK"),
            content_type: String::from("text/plain"),
            content: String::new(),
        }
    }

    pub fn set_content_from_file(&mut self, file_path: &str) {
        let mut file = match File::open(file_path) {
            Ok(file) => file,
            Err(_) => {
                self.status = String::from("404 Not Found");
                return;
            }
        };

        let mut content = String::new();
        if let Err(_) = file.read_to_string(&mut content) {
            self.status = String::from("500 Internal Server Error");
            return;
        }

        self.content_type = String::from("text/html");
        self.content = content;
    }

    pub fn set_content(&mut self, con: &str) {
        self.content = String::from(con);
    }

}

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    let mut httpresp = HTTPResponse::new();
    let mut response_str = String::new();
    let mut response = "";
    match stream.read(&mut buffer) {
        Ok(_) => {
            let request = String::from_utf8_lossy(&buffer);

            let headers: Vec<&str> = request.split(utils::CRLF).collect();

            if let Some(request_line) = headers.get(0) {
                let headers_parts: Vec<&str> = request_line.split_whitespace().collect();

                if headers_parts.len() >= 2 {

                    let path = headers_parts[1];
                    println!("Requested path: {}", path);

                    if path == "/" {
                        httpresp.set_content_from_file("C:/Users/randr/Documents/Rust/HTTPRemake/src/html/main.html");
                        response_str = format!("{} {} {}\r\n\r\n{}", httpresp.version, httpresp.status, httpresp.content_type, httpresp.content);
                        response = response_str.as_str();
                        println!("{}", response);
                    } else if path == "/teste" {
                        httpresp.set_content("Teste!");
                        response_str = format!("{} {} {}\r\n\r\n{}", httpresp.version, httpresp.status, httpresp.content_type, httpresp.content);
                        response = response_str.as_str();
                    } else if path == "/novo" {
                        httpresp.set_content("Novo!");
                        response_str = format!("{} {} {}\r\n\r\n{}", httpresp.version, httpresp.status, httpresp.content_type, httpresp.content);
                        response = response_str.as_str();
                    } else {
                        httpresp.set_content("Page not found! :(");
                        response_str = format!("{} {} {}\r\n\r\n{}", httpresp.version, httpresp.status, httpresp.content_type, httpresp.content);
                        response = response_str.as_str();
                    }
                }

            }

            stream.write(response.as_bytes()).unwrap();
            stream.flush().unwrap();
        }
        Err(e) => {
            println!("Failed to read from the stream: {}", e);
        }
    }
}


fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:5455").unwrap();
    println!("Listening on 127.0.0.1:5455");
    println!("Available paths: ");
    println!("/");
    println!("/teste");
    println!("/novo");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| {
                    handle_client(stream);
                });
            }
            Err(e) => {
                println!("Unable to connect: {}", e);
            }
        }
    }
    Ok(())
}