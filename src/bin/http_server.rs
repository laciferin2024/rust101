use std::fmt::format;
use std::{fs, thread};
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::time::Duration;

fn main(){
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    let pool = rust101::ThreadPool::new(4);

    // Terminates after 2 takes: Drop::drop is called
    for stream in listener.incoming().take(2)
    {
        let stream  = stream.unwrap();
        println!("Connection Established");

        pool.execute(||{
            handle_connection(stream);
        })
    }

    print!("Shutting down");




}

fn handle_connection(mut stream: TcpStream){
    let mut buffer: [u8;1024] = [0;1024];

    stream.read(&mut buffer).unwrap();
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    let get = b"GET / HTTP/1.1\r\n";

    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status, inputFile) =
        if buffer.starts_with(get) {
            ("200 OK","index.html")
            // every statement is separated by a carriage return \r and line \n
        }else if buffer.starts_with(sleep){
            println!("sleep");
            thread::sleep(Duration::from_secs(5));
            ("200 OK", "index.html")
        }
        else {
            ("404 NOT FOUND", "404.html")
        };

    let htmlContent = fs::read_to_string(inputFile).unwrap();
    let response = format!("HTTP/1.1 {}\r\nContent-Length: {}\r\n\r\n{}",status, htmlContent.len(), htmlContent);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}