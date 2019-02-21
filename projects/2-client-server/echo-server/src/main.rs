extern crate clap;
extern crate log;
extern crate simple_logger;

use clap::{App, Arg};
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::{io, thread};

use log::{error, info};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::time::Duration;

fn main() -> io::Result<()> {
    // Initialize logger
    simple_logger::init().expect("could not initialize logger");

    let matches = App::new("Simple Echo Server")
        .version("1.0")
        .author("Fabian Huch <fabian.huch@qaware.de>")
        .arg(
            Arg::with_name("PORT")
                .required(true)
                .validator(|s| s.parse::<u16>().and(Ok(())).map_err(|e| e.to_string())),
        )
        .get_matches();

    let port = matches.value_of("PORT").unwrap().parse::<u16>().unwrap();
    let localhost_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), port);
    let listener = TcpListener::bind(localhost_addr)?;

    let msg_history = Arc::new(Mutex::new(Vec::<String>::new()));

    // Wait for incoming connections
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                info!("Connection accepted.");

                // Cloning the Arc will get a new pointer to the same underlying mutex on the data
                let msg_history = Arc::clone(&msg_history);

                // Spawn new thread for every connection
                thread::spawn(move || match echo(stream, msg_history) {
                    Ok(_) => info!("Connection handled successfully."),
                    Err(e) => error!("Error during connection: {}", e),
                });
            }
            Err(e) => error!("Connection request failed. Cause: {}", e),
        }
    }

    Ok(())
}

/// Echo-server functionality
fn echo(mut stream: TcpStream, history: Arc<Mutex<Vec<String>>>) -> io::Result<()> {
    let mut reader = BufReader::new(stream.try_clone()?);

    loop {
        // Read line
        let mut line = String::new();
        reader.read_line(&mut line)?;
        let is_eof = !line.ends_with('\n');

        match history.lock() {
            Ok(mut data) => {
                // Add new message and print the last three out
                info!("Received {:?}", line);
                data.insert(0, line);
                let last_msgs: Vec<&String> = data.iter().take(3).collect();
                info!("Last three received messages: {:#?}", last_msgs);
                // Echo string
                stream.write_all(last_msgs[0].as_bytes())?;
                stream.flush()?;
            }
            Err(e) => error!("Could not access history. Cause: {}", e),
        }

        if is_eof {
            break;
        }
    }

    Ok(())
}
