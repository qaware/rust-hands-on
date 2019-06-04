use std::io;
use std::io::ErrorKind::{TimedOut, WouldBlock};
use std::io::{BufRead, BufReader, Write};
use std::net::{Shutdown, TcpStream};

/// Connection encapsulation
pub struct Connection<R: BufRead, W: Write> {
    reader: R,
    writer: W,
}
/// Implementation over generic reader/writer for better testability
impl<R: BufRead, W: Write> Connection<R, W> {
    /// Sends message
    pub fn send_message(&mut self, message: &str) -> io::Result<()> {
        self.writer.write_all(message.as_bytes())?;
        self.writer.flush()
    }
    /// Receives a single line, i.e. reads until \n, Timeout, or EOF.
    pub fn receive_line(&mut self) -> io::Result<Option<String>> {
        let mut buf = String::new();
        match self.reader.read_line(&mut buf) {
            // 0 bytes means no delimiter read -> is EOF
            Ok(0) => Ok(None),
            Ok(_) => Ok(Some(buf)),
            Err(e) => match e.kind() {
                // Timed out -> no answer
                WouldBlock | TimedOut => Ok(None),
                _ => return Err(e),
            },
        }
    }
}
/// Impl on tcp stream for construction / terminating of write part
impl Connection<BufReader<TcpStream>, TcpStream> {
    pub fn new(stream: TcpStream) -> Self {
        let reader = BufReader::new(stream.try_clone().expect("tcp stream must be cloneable"));
        Self {
            reader,
            writer: stream,
        }
    }
    /// Consumes the connection, returning the remaining read data
    pub fn receive_remaining(mut self) -> io::Result<Option<String>> {
        // Shut down writer first
        self.writer.shutdown(Shutdown::Write)?;

        // Read all remaining lines
        let mut lines = String::new();
        loop {
            match self.receive_line()? {
                Some(line) => lines.push_str(&line),
                None => break,
            }
        }

        if lines.is_empty() {
            Ok(None)
        } else {
            Ok(Some(lines))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Connection;

    #[test]
    fn test_send_line() {
        let mut write_buffer = [0u8; 10];

        let result = Connection {
            reader: "".as_bytes(),
            writer: write_buffer.as_mut(),
        }
        .send_message("test\n");

        assert!(result.is_ok());
        assert_eq!(b"test\n", &write_buffer[..5]);
        assert_eq!([0, 0, 0, 0, 0], &write_buffer[5..]);
    }

    #[test]
    fn test_receive_empty_line() {
        let mut write_buffer = [0u8; 0];

        let mut connection = Connection {
            reader: "\n".as_bytes(),
            writer: write_buffer.as_mut(),
        };

        // Empty line
        let mut result = connection.receive_line();
        assert_eq!("\n", result.unwrap().unwrap().as_str());

        // Nothing to read
        result = connection.receive_line();
        assert!(result.unwrap().is_none());
    }

    #[test]
    fn test_receive_lines() {
        let mut write_buffer = [0u8; 0];

        let mut connection = Connection {
            reader: "line1\nline2".as_bytes(),
            writer: write_buffer.as_mut(),
        };

        // First line
        let mut result = connection.receive_line();
        assert_eq!("line1\n", result.unwrap().unwrap().as_str());

        // Second line
        result = connection.receive_line();
        assert_eq!("line2", result.unwrap().unwrap().as_str());

        // Nothing to read
        result = connection.receive_line();
        assert!(result.unwrap().is_none());
    }
}
