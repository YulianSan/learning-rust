use std::{
    env::current_dir,
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8000").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("Connection established!");

        handle_request(stream)
    }
}

fn handle_request(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let _http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    // println!("Request: {_http_request:#?}");

    let current_dir = current_dir().unwrap();
    let status_line = "HTTP/2.0 200 OK";
    let file_path = current_dir.join("public").join("hello.html");
    let mut file_content = fs::File::open(file_path).unwrap();
    let mut contents = String::new();
    file_content.read_to_string(&mut contents).unwrap();
    let length = contents.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}
