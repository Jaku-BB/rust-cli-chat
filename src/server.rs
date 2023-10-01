use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{SocketAddrV4, TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

// TODO: Allow user to specify custom name, then use it as an identifier
type StreamMap = Arc<Mutex<HashMap<usize, TcpStream>>>;

pub(crate) fn initialize_server(address: SocketAddrV4) {
    let listener = TcpListener::bind(address).expect("Should create a TCP server");

    println!("Listening on {}...", address);

    let mut next_stream_index: usize = 0;
    let stream_map: StreamMap = Arc::new(Mutex::new(HashMap::new()));

    loop {
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    let stream_map_clone = stream_map.clone();

                    thread::spawn(move || {
                        handle_stream(stream, next_stream_index, stream_map_clone);
                    });
                }
                Err(error) => eprintln!("Couldn't establish connection: {}", error),
            }

            next_stream_index += 1;
        }
    }
}

fn handle_stream(mut stream: TcpStream, index: usize, stream_map: StreamMap) {
    let stream_clone = stream.try_clone().expect("Should clone the TCP stream");

    stream_map
        .lock()
        .expect("Should lock stream map")
        .insert(index, stream_clone);

    loop {
        let mut message_buffer = vec![0; 256];

        match stream.read(&mut message_buffer) {
            Ok(0) => {
                println!("Stream shut down, index: {}", index);

                stream_map
                    .lock()
                    .expect("Should lock stream map")
                    .remove(&index);

                break;
            }
            Ok(_) => {
                match String::from_utf8(message_buffer) {
                    Ok(message) => {
                        let message_with_stream_index = format!("Client {}: {}", index, message);
                        write_to_all(stream_map.clone(), message_with_stream_index);
                    }
                    Err(error) => eprintln!("Message isn't in UTF-8 format: {}", error),
                };
            }
            Err(error) => eprintln!("Couldn't read from the TCP stream: {}", error),
        }
    }
}

fn write_to_all(stream_map: StreamMap, message: String) {
    for (_, stream) in stream_map
        .lock()
        .expect("Should lock stream map")
        .iter_mut()
    {
        // TODO: This shouldn't panic, should it?
        stream
            .write(message.as_bytes())
            .expect("Should write to the TCP stream");
    }
}
