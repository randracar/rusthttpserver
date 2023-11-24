use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::io;
use std::fs::File;
use std::thread;

mod utils;
 
pub struct HTTPResponse {
    version: String,
    status: String,
    content_type: String,
    content: String,
}

fn log_error(file_path: &str, error: String) -> io::Result<()> {
    // Open the file in write mode, creating it if it doesn't exist
    let mut file = File::create(file_path)?;

    let content = format!("{}\n", error);
    // Write the content to the file
    file.write_all(content.as_bytes())?;

    Ok(())
}

impl HTTPResponse {
    pub fn new() -> Self {
        Self {
            version: String::from(utils::HTTP11),
            status: String::from(utils::STATUS200),
            content_type: String::from(utils::TEXTPLAIN),
            content: String::new(),
        }
    }

    pub fn set_content_from_file(&mut self, file_path: &str) {
        let mut file = match File::open(file_path) {
            Ok(file) => file,
            Err(e) => {
                self.status = format!("{}: {}", utils::ERROR404, e);
                log_error(utils::ERRORLOGS, self.status.clone());
                println!("{}", self.status);
                return;
            }
        };

        let mut content = String::new();
        if let Err(e) = file.read_to_string(&mut content) {
            self.status = format!("{}: {}", utils::ERROR500, e);
            log_error(utils::ERRORLOGS, self.status.clone());
            return;
        }

        self.content_type = String::from(utils::HTML);
        self.content = content;
    }

    pub fn set_content(&mut self, con: &str) {
        self.content = String::from(con);
    }

}


fn handle_path(path: &str, httpresp: HTTPResponse) -> String {
    let mut new_httpresp = httpresp;
    if path == "/" {
        new_httpresp.set_content_from_file(utils::MAINSITE);
        let response_str = format!("{} {} {}\r\n\r\n{}", new_httpresp.version, new_httpresp.status, new_httpresp.content_type, new_httpresp.content);
        return response_str;
    } else if path == "/teste" {
        new_httpresp.set_content(utils::TESTE_CONTENT);
        let response_str = format!("{} {} {}\r\n\r\n{}", new_httpresp.version, new_httpresp.status, new_httpresp.content_type, new_httpresp.content);
        return response_str;
    } else if path == "/novo" {
        new_httpresp.set_content(utils::NOVO_CONTENT);
        let response_str = format!("{} {} {}\r\n\r\n{}", new_httpresp.version, new_httpresp.status, new_httpresp.content_type, new_httpresp.content);
        return response_str;
    } else {
        new_httpresp.set_content(utils::PAGENOTFOUND);
        let response_str = format!("{} {} {}\r\n\r\n{}", new_httpresp.version, new_httpresp.status, new_httpresp.content_type, new_httpresp.content);
        return response_str;
    }
}

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    let  httpresp = HTTPResponse::new();
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
                    response_str = handle_path(path, httpresp);
                    response = response_str.as_str();
                }

            }

            stream.write(response.as_bytes()).unwrap();
            stream.flush().unwrap();
        }
        Err(e) => {
            let err = format!("{} {}", utils::STREAMREADERROR, e);
            println!("{}", err);
            log_error(utils::ERRORLOGS, err);
        }
    }
}


fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind(utils::IPADDR).unwrap();
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
                let err = format!("{} {}", utils::UNABLETOCONNECT, e);
                log_error(utils::ERRORLOGS, err);
            }
        }
    }
    Ok(())
}