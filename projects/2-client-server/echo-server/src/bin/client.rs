extern crate log;
extern crate simple_logger;

use std::io;
use std::net::TcpStream;
use std::io::Write;
use std::io::Read;
use log::info;
use std::net::Shutdown;

const SERVER_ADDRESS: &str = "127.0.0.1:6666";

fn main() -> io::Result<()> {
    // Initialize logger
    simple_logger::init().expect("Could not initialize logger");

    // Connect to server
    let mut stream = TcpStream::connect(SERVER_ADDRESS)?;

    let message = String::from("Hello world");

    // Write message and EOF
    stream.write_all(message.as_bytes())?;
    stream.shutdown(Shutdown::Write)?;

    // Read result
    let mut buf = String::new();
    stream.read_to_string(&mut buf)?;

    info!("Wrote \"{}\", received \"{}\"", message, buf);

    Ok(())
}