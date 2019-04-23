extern crate clap;
extern crate log;
extern crate simple_logger;

use std::io;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::{Shutdown, TcpStream, ToSocketAddrs};

use clap::{App, Arg};
use log::{debug, info};
use std::io::ErrorKind::{TimedOut, WouldBlock};
use std::time::Duration;
use std::str::from_utf8;

const READ_TIMEOUT_MS: u64 = 100;
const MESSAGE: [&str; 7] = [
    "HELO client.example.com\n",
    "MAIL FROM:<mail@samlogic.com>\n",
    "RCPT TO:<john@mail.com>\n",
    "DATA\n",
    "<The message data (body text, subject, e-mail header, attachments etc) is sent>\n",
    ".\n",
    "QUIT",
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
    let mut stream = TcpStream::connect(matches.value_of("ADDRESS").unwrap())?;
    stream.set_read_timeout(Some(Duration::from_millis(READ_TIMEOUT_MS)))?;
    stream.set_write_timeout(None)?;

    // Reader to read single lines
    let mut reader = BufReader::new(stream.try_clone()?);

    // Receive initial message, if any:
    match receive_line(&mut reader)? {
        Some(res) => info!("Received message: {:?}", res),
        None => (),
    }

    for line in MESSAGE.iter() {
        // Write linAdded history to echo servere
        stream.write_all(line.as_bytes())?;

        // Shutdown write stream if last line was written
        if line == MESSAGE.last().unwrap() {
            stream.shutdown(Shutdown::Write)?;
        }

        // Receive answer, if any
        match receive_line(&mut reader)? {
            Some(res) => info!("Wrote {:?}, received {:?}", line, res),
            None => info!("Wrote {:?}, received nothing", line),
        }
    }

    // Read until EOF, to check if there is more after line ending
    let mut remain = Vec::new();
    match reader.read_to_end(&mut remain) {
        Ok(0) | Err(_) => (),
        Ok(_) => info!("Finally received {:?}", from_utf8(remain.as_slice()).expect("No utf8 str")),
    }

    Ok(())
}

/// Receives a single line, i.e. reads until \n, Timeout, or EOF.
fn receive_line(reader: &mut BufRead) -> io::Result<Option<String>> {
    let mut buf = String::new();
    match reader.read_line(&mut buf) {
        Ok(_) => Ok(Some(buf)),
        Err(e) => match e.kind() {
            // Timed out -> no answer
            WouldBlock | TimedOut => Ok(None),
            _ => return Err(e),
        },
    }
}
