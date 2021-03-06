use std::io::BufReader;
use std::net::{TcpListener, TcpStream};
use std::{io, thread};

fn main() {
    let port = 1337;
    let listener = TcpListener::bind(("localhost", port)).expect("Port already in use!");

    // Wait for incoming connections
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // Spawn new thread for every connection
                thread::spawn(move || match handle_connection(stream) {
                    Ok(_) => println!("Connection handled successfully."),
                    Err(e) => println!("Oh no. Error during connection: {}", e),
                });
            }
            Err(e) => println!("Oh no. Connection request failed. Cause: {}", e),
        }
    }
}

fn handle_connection(mut stream: TcpStream) -> io::Result<()> {
    // Make a second stream to read and write in parallel
    let mut reader = BufReader::new(stream.try_clone()?);

    // TODO: implement echo functionality, reading from reader and writing to stream

    Ok(())
}
