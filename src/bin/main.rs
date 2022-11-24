// Most of the code is based off the Rust Lang Book

use std::{
    net::{TcpListener, TcpStream},
    io::{prelude::*, BufReader},
    fs,
    thread,
    time::Duration
};
use TCP_Network::ThreadPool;


fn main() {
    // Need to update the address where would it bind
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}


fn handle_connection(mut stream: TcpStream) {

    

    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();
    println!("{}",request_line);

    if request_line == "GET / HTTP/1.1"{
        let status_line = "HTTP/1.1 200 OK";
        let filename = "src/hello.html";
        let contents = fs::read_to_string(filename).unwrap();
        let response = format!("{}\r\nContent-Length: {}\r\n\r\n{}", status_line, contents.len(), contents);
        stream.write_all(response.as_bytes()).unwrap();

    } else if request_line == "GET /sleep HTTP/1.1"{
        thread::sleep(Duration::from_secs(5));
        let status_line = "HTTP/1.1 200 OK";
        let filename = "src/hello.html";
        let contents = fs::read_to_string(filename).unwrap();
        let response = format!("{}\r\nContent-Length: {}\r\n\r\n{}", status_line, contents.len(), contents);
        stream.write_all(response.as_bytes()).unwrap();

    } 
    else if request_line == "GET /Resume.pdf HTTP/1.1"{

        // Read the file and get the contents as utf8 format
        let status_line = "HTTP/1.1 200 OK";
        let buf_content = fs::read("src/Resume.pdf").unwrap();
        let contents = unsafe {String::from_utf8_unchecked(buf_content)};
        let length = contents.len();

        // Send the message/response to the client
        let response = format!("{status_line}\r\n\
        Content-Disposition: attachment; filename=\"Resume.pdf\"\r\n\
        Content-Type: application/octet-stream\r\n\
        Content-Length: {length}\r\n\r\n{contents}");
        println!("{}",response);
        stream.write_all(response.as_bytes()).unwrap();
        
    } 
    else if request_line == "GET /Resume.txt HTTP/1.1"{
        let status_line = "HTTP/1.1 200 OK";
        let buf_content = fs::read("src/Resume.txt").unwrap();
        let contents = unsafe {String::from_utf8_unchecked(buf_content)};
        let length = contents.len();

        let response = format!("{status_line}\r\n\
        Content-Disposition: attachment; filename=\"Resume.txt\"\r\n\
        Content-Type: application/octet-stream\r\n\
        Content-Length: {length}\r\n\r\n{contents}");
        println!("{}",response);
        stream.write_all(response.as_bytes()).unwrap();
        
    } else {
        let status_line = "HTTP/1.1 404 NOT FOUND";
        let filename = "src/404.html";
        let contents = fs::read_to_string(filename).unwrap();
        let response = format!("{}\r\nContent-Length: {}\r\n\r\n{}", status_line, contents.len(), contents);
        stream.write_all(response.as_bytes()).unwrap();
    }

    // let (status_line, filename) = match &request_line[..]{
    //     "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "src/hello.html"),
    //     "GET /sleep HTTP/1.1" => {
    //         thread::sleep(Duration::from_secs(5));
    //         ("HTTP/1.1 200 OK", "src/hello.html")
    //     },
    //     "GET /Resume.pdf HTTP/1.1" => {
    //         let buf_content = fs::read("src/Resume.pdf").unwrap();
    //         let contents = unsafe {String::from_utf8_unchecked(buf_content)};
    //         let length = contents.len();
    //     },
    //     _ => ("HTTP/1.1 404 NOT FOUND", "src/404.html")
    // };

    

}
