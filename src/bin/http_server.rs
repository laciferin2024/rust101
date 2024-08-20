use std::fmt::format;
use std::fs;
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};

fn main(){
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming()
    {
        let stream  = stream.unwrap();
        println!("Connection Established");
        handle_connection(stream);
    }




}

fn handle_connection(mut stream: TcpStream){
    let mut buffer: [u8;1024] = [0;1024];

    stream.read(&mut buffer).unwrap();
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    let acceptedReq = b"GET / HTTP/1.1\r\n";
    let mut inputFile;
    if buffer.starts_with(acceptedReq) {
        inputFile = "index.html";
        // every statement is separated by a carriage return \r and line \n
    }else {
        inputFile = "404.html";
    }
    let htmlContent = fs::read_to_string(inputFile).unwrap();
    let response = format!("HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}", htmlContent.len(), htmlContent);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}