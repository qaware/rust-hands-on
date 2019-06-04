use std::io;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;
use std::sync::{Arc, Mutex};

use log::info;

/// Connection encapsulation
pub struct Connection<R: BufRead, W: Write> {
    reader: R,
    writer: W,
}
/// Implementation over generic reader/writer for testability
impl<R: BufRead, W: Write> Connection<R, W> {
    /// Echo-server functionality. Appends to history.
    pub fn echo(mut self, history: Arc<Mutex<Vec<String>>>) -> io::Result<()> {
        for line in self.reader.lines() {
            let mut data = history.lock().unwrap();
            // Add new message and print the last three out
            data.push(line?);
            let last_msgs: Vec<&str> = data.iter().map(AsRef::as_ref).rev().take(3).collect();
            info!("Last three received messages: {:#?}", last_msgs);
            // Echo string
            self.writer.write_all(last_msgs[0].as_bytes())?;
            self.writer.write_all("\n".as_bytes())?;
            self.writer.flush()?;
        }

        Ok(())
    }
}
/// Additional implementation over tcp stream allows constructing connection from it
impl Connection<BufReader<TcpStream>, TcpStream> {
    pub fn new(stream: TcpStream) -> Self {
        let reader = BufReader::new(stream.try_clone().expect("Cannot clone tcp stream"));
        Self {
            reader,
            writer: stream,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use super::Connection;

    #[test]
    fn test_echo_single_line() {
        let mut write_buffer = [0u8; 10];
        let history = Arc::new(Mutex::from(Vec::new()));

        let result = Connection {
            reader: "echo!".as_bytes(),
            writer: write_buffer.as_mut(),
        }
        .echo(Arc::clone(&history));

        assert!(result.is_ok());

        // Check output stream
        assert_eq!(b"echo!\n", &write_buffer[..6]);
        assert_eq!([0, 0, 0, 0], &write_buffer[6..]);

        // Check history
        let history_vec = history.lock().unwrap();
        assert_eq!(*history_vec, vec!["echo!"]);
    }

    #[test]
    fn test_echo_lines() {
        let mut write_buffer = [0u8; 15];
        let history = Arc::new(Mutex::from(Vec::new()));

        let result = Connection {
            reader: "haaalo \necho!".as_bytes(),
            writer: write_buffer.as_mut(),
        }
        .echo(Arc::clone(&history));

        assert!(result.is_ok());

        // Check output stream
        assert_eq!(b"haaalo \necho!\n", &write_buffer[..14]);

        // Check history
        let history_vec = history.lock().unwrap();
        assert_eq!(*history_vec, vec!["haaalo ", "echo!"]);
    }
}
