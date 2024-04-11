use std::{
    fs,
    io::{prelude::*, BufReader, Result},
    net::{TcpListener, TcpStream},
    path::PathBuf,
    thread,
};

use simple_http_project::ThreadPool;

struct Page {
    file_path: PathBuf,
    status_line: String,
}

impl Page {
    fn render(&self, stream: &mut TcpStream) -> Result<()> {
        let contents = fs::read_to_string(&self.file_path).unwrap();
        let length = contents.len();
        let response = format!(
            "{}\r\nContent-Length: {}\r\n\r\n{}",
            self.status_line, length, contents
        );

        Ok(stream.write_all(response.as_bytes())?)
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8000").unwrap();
    let pool = ThreadPool::new(4);
    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();
        println!("Connection established!");

        pool.execute(|| handle_request(stream));
        // using infinite numbers of threads
        // thread::spawn(|| handle_request(stream));
    }
}

fn handle_request(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();
    // let http_request: Vec<_> = buf_reader
    //     .lines()
    //     .map(|result| result.unwrap())
    //     .take_while(|line| !line.is_empty())
    //     .collect();
    //
    // println!("Request: {http_request:#?}");

    let (file_path, status_line) = match request_line.as_str() {
        "GET / HTTP/1.1" => ("public/hello.html", "HTTP/2.0 200 OK"),
        _ => {
            thread::sleep(std::time::Duration::from_secs(5));
            ("public/errors/404.html", "HTTP/2.0 404 NOT FOUND")
        }
    };

    let page = Page {
        file_path: file_path.into(),
        status_line: status_line.to_string(),
    };

    page.render(&mut stream).unwrap();
}

// fn index(stream: &mut TcpStream) {
//     let current_dir = current_dir().unwrap();
//     let file_path = current_dir.join("public").join("hello.html");
//
//     let status_line = "HTTP/2.0 200 OK";
//     let mut file_content = fs::File::open(file_path).unwrap();
//     let mut contents = String::new();
//     file_content.read_to_string(&mut contents).unwrap();
//     let length = contents.len();
//
//     let response = format!("{status_line}\r\nContent-Length: {length}\r\n{contents}");
//
//     stream.write_all(response.as_bytes()).unwrap();
// }
//
// fn not_found(stream: &mut TcpStream) {
//     let current_dir = current_dir().unwrap();
//     let file_path = current_dir.join("public").join("errors").join("404.html");
//
//     let status_line = "HTTP/2.0 404 NOT FOUND";
//     let contents = fs::read_to_string(file_path).unwrap();
//     let length = contents.len();
//
//     let response = format!("{status_line}\r\nContent-Length: {length}\r\n{contents}");
//
//     stream.write_all(response.as_bytes()).unwrap();
// }
