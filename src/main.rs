use std::io::{TcpListener, TcpStream};
use std::io::{Acceptor, Listener};


fn main() {
    let server_address = "127.0.0.1";
    let server_port    = 8001;
    let listener = TcpListener::bind(server_address, server_port);

    let mut acceptor = listener.listen();

    fn handle_client(mut stream: TcpStream) {
        let mut buffer: Vec<u8> = Vec::from_fn( 10240u, |_| 0 );
        let bytes = stream.read(buffer.as_mut_slice());
        println!("Bytes: {}", bytes.unwrap());
        println!("Raw:\n{}", String::from_utf8(buffer).unwrap());
    }

    println!("Server bound to: {}:{}", server_address, server_port);
    for stream in acceptor.incoming() {
        match stream {
            Err(e) => { println!("failed: {}", e) }
            Ok(stream) => spawn(proc() {
                handle_client(stream);
            })
        }
    }

    drop(acceptor);
}
