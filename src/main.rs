use std::{io::Read, net::{TcpListener, TcpStream}};

const LOCAL_HOST: &str = "127.0.0.1:7878";

fn handle_connection(stream: TcpStream) {
    let mut buffer: [u8; 1024] = [0; 1024];
    let get = "GET / HTTP/1.1";
    let request = stream.read(&mut buffer);

    
}
fn main() {
    let listener = TcpListener::bind(LOCAL_HOST);
    println!("Listening on {}", LOCAL_HOST);
    for stream in listener.unwrap().incoming() {
            handle_connection(stream);
    }
}
