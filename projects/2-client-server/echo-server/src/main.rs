extern crate log;
extern crate simple_logger;

use log::{error, info};
use std::io::{Read, Write};
use std::net::Shutdown;
use std::net::TcpListener;
use std::net::TcpStream;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::Duration;
use std::{io, thread};

const SERVER_ADDRESS: &str = "127.0.0.1:6666";

fn main() -> io::Result<()> {
    // Initialize logger
    simple_logger::init().expect("could not initialize logger");

    let listener = TcpListener::bind(SERVER_ADDRESS)?;

    let msg_history = Arc::new(Mutex::new(Vec::<String>::new()));

    // Wait for incoming connections
    for stream in listener.incoming() {
        // Cloning the Arc will get a new pointer to the same underlying mutex on the data
        let msg_history = Arc::clone(&msg_history);

        // Spawn new thread for every connection
        thread::spawn(move || match stream {
            Ok(stream) => {
                info!("Connection accepted.");

                match echo(stream, msg_history) {
                    Ok(_) => info!("Connection handled successfully."),
                    Err(e) => error!("Error during connection: {}", e),
                }
            }
            Err(e) => error!("Connection request failed. Cause: {}", e),
        });
    }

    Ok(())
}

/// Echo-server functionality
fn echo(mut stream: TcpStream, history: Arc<Mutex<Vec<String>>>) -> io::Result<()> {
    let mut buf = String::new();
    stream.read_to_string(&mut buf)?;

    // Some heavy load here!
    thread::sleep(Duration::from_secs(10));

    // Add new message and print the last three out
    match history.lock() {
        Ok(mut data) => {
            data.push(buf.clone());
            let last_msgs: Vec<&String> = data.iter().take(3).collect();
            info!("Last three received messages: {:?}", last_msgs)
        }
        Err(e) => error!("Could not access history. Cause: {}", e),
    }

    // Echo string and write EOF
    stream.write_all(buf.as_bytes())?;
    stream.shutdown(Shutdown::Both)
}
