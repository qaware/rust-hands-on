extern crate log;
extern crate simple_logger;

use std::{io, thread};
use std::time::Duration;
use std::net::TcpStream;
use std::net::TcpListener;
use log::{info, error};
use std::io::{Read, Write};
use std::net::Shutdown;
use std::sync::Mutex;
use std::sync::Arc;

const SERVER_ADDRESS: &str = "127.0.0.1:6666";

fn main() -> io::Result<()> {
    // Initialize logger
    simple_logger::init().expect("could not initialize logger");

    let listener = TcpListener::bind(SERVER_ADDRESS)?;

    let message_history: Vec<String> = Arc::new(Mutex::new(vec![]));

    // Wait for incoming connections
    for stream in listener.incoming() {
        // Spawn new thread for every connection
        thread::spawn(move || {
            match stream {
                Ok(mut stream) => {
                    info!("Connection accepted.");

                    match echo(&mut stream) {
                        Ok(_) => {
                            info!("Connection handled successfully.")
                        },
                        Err(e) => {
                            error!("Error during connection: {}", e)
                        }
                    }
                }
                Err(e) => {
                    error!("Connection request failed. Cause: {}", e)
                }
            }
        });
    }

    Ok(())
}

/// Echo-server functionality
fn echo(stream: &mut TcpStream, history: Arc<Vec<String>>) -> io::Result<()> {
    let mut buf = String::new();
    stream.read_to_string(&mut buf)?;

    // Some heavy load here!
    thread::sleep(Duration::from_secs(10));

    info!("Received message: {}", buf);

    stream.write_all(buf.as_bytes())?;
    stream.shutdown(Shutdown::Both)
}
