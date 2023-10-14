use std::io::{self, Read, Write};
use std::net::{SocketAddrV4, TcpStream};
use std::process::exit;
use std::thread;

pub(crate) fn initialize_client(address: SocketAddrV4) {
    let mut stream = TcpStream::connect(address).expect("Should connect to the TCP server");
    let mut stream_clone = stream.try_clone().expect("Should clone the TCP stream");

    println!("Hello! Current server address: {}", address);
    println!("You can type your message now.");

    thread::spawn(move || loop {
        let mut message_buffer = vec![0; 256];

        match stream_clone.read(&mut message_buffer) {
            Ok(0) => {
                println!("Server shut down...");
                exit(0);
            }
            Ok(_) => {
                match String::from_utf8(message_buffer) {
                    Ok(message) => println!("{}", message),
                    Err(error) => eprintln!("Message isn't in UTF-8 format: {}", error),
                };
            }
            Err(error) => eprintln!("Couldn't read from the TCP stream: {}", error),
        }
    });

    loop {
        let message = get_user_input();

        match stream.write(message.as_bytes()) {
            Ok(_) => {}
            Err(error) => eprintln!("Couldn't send message: {}", error),
        }
    }
}

fn get_user_input() -> String {
    let mut message = String::new();

    io::stdin()
        .read_line(&mut message)
        .expect("Should read from standard input");

    return message.trim().to_string();
}
