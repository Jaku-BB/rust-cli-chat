use std::net::{TcpListener, TcpStream};
use std::thread;

pub(crate) fn initialize_server(address: &str) {
    let listener = match TcpListener::bind(address) {
        Ok(listener) => listener,
        Err(error) => panic!("Couldn't bind to address: {}", error),
    };

    println!("Listening on {address}...");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let stream_thread = thread::spawn(move || handle_tcp_stream(stream));
                stream_thread.join().expect("Stream thread panicked!");
            }
            Err(error) => eprintln!("Couldn't establish connection: {}", error),
        }
    }
}

fn handle_tcp_stream(mut stream: TcpStream) {
    println!("Connection established!");
}
