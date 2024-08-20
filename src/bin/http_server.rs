use std::net::{TcpListener, TcpStream};
use std::io::Read;

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
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]))
}