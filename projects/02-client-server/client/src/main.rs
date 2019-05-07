extern crate clap;
extern crate log;
extern crate simple_logger;

use std::io;
use std::net::{TcpStream, ToSocketAddrs};
use std::time::Duration;

use clap::{App, Arg};
use log::info;

use client::Connection;

const READ_TIMEOUT_MS: u64 = 100;
const MESSAGE: [&str; 7] = [
    "HELO client.example.com\n",
    "MAIL FROM:<mail@samlogic.com>\n",
    "RCPT TO:<john@mail.com>\n",
    "DATA\n",
    "<The message data (body text, subject, e-mail header, attachments etc) is sent>\n",
    ".\n",
    "QUIT\n",
];

fn main() -> io::Result<()> {
    let matches = App::new("SMTP mock client")
        .version("1.0")
        .author("Fabian Huch <fabian.huch@qaware.de>")
        .arg(
            Arg::with_name("ADDRESS")
                .required(true)
                .validator(|s| s.to_socket_addrs().and(Ok(())).map_err(|e| e.to_string())),
        )
        .get_matches();

    // Initialize logger
    simple_logger::init().expect("Could not initialize logger");

    // Connect to server
    let stream = TcpStream::connect(matches.value_of("ADDRESS").unwrap())?;
    stream.set_read_timeout(Some(Duration::from_millis(READ_TIMEOUT_MS)))?;
    stream.set_write_timeout(None)?;

    let mut connection = Connection::new(stream);

    // Receive initial message(s), if any:
    loop {
        match connection.receive_line()? {
            Some(res) => info!("Received message: {:?}", res),
            None => break,
        }
    }

    // Send lines and receive answers, if any:
    for line in MESSAGE.iter() {
        // Write line and receive answer, if any
        connection.send_message(line)?;
        match connection.receive_line()? {
            Some(res) => info!("Wrote {:?}, received {:?}", line, res),
            None => info!("Wrote {:?}, received nothing", line),
        }
    }

    // Receive remaining messages, if any
    match connection.receive_remaining()? {
        Some(res) => info!("Finally received {:?}", res),
        None => (),
    }

    Ok(())
}
