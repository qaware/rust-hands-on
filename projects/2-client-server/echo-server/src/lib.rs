use std::io;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;
use std::sync::{Arc, Mutex};

use log::{error, info};

/// Connection encapsulation for better testability
pub struct Connection<R: BufRead, W: Write> {
    reader: R,
    writer: W,
}
impl<R: BufRead, W: Write> Connection<R, W> {
    /// Echo-server functionality. Appends to history.
    pub fn echo(mut self, history: Arc<Mutex<Vec<String>>>) -> io::Result<()> {
        for line in self.reader.lines() {
            match history.lock() {
                Ok(mut data) => {
                    // Add new message and print the last three out
                    data.push(line?);
                    let last_msgs: Vec<&String> = data.iter().rev().take(3).collect();
                    info!("Last three received messages: {:#?}", last_msgs);
                    // Echo string
                    self.writer.write_all(last_msgs[0].as_bytes())?;
                    self.writer.write_all("\n".as_bytes())?;
                    self.writer.flush()?;
                }
                Err(e) => error!("Could not access history. Cause: {}", e),
            }
        }

        Ok(())
    }
}
/// Implementation on tcp stream - has a specific type here!
impl Connection<BufReader<TcpStream>, TcpStream> {
    pub fn new(stream: TcpStream) -> Self {
        let reader = BufReader::new(stream.try_clone().expect(""));
        Self {
            reader,
            writer: stream,
        }
    }
}

#[cfg(test)]
mod test {
    use std::str;
    use std::sync::{Arc, Mutex};

    use super::Connection;

    #[test]
    fn test_echo_single_line() {
        let mut write_buffer = [0u8; 10];
        let history = Arc::new(Mutex::from(Vec::new()));

        let result = Connection {
            reader: "echo!".as_bytes(),
            writer: &mut write_buffer.as_mut(),
        }
        .echo(Arc::clone(&history));

        assert!(result.is_ok());

        // Check output stream
        assert_eq!("echo!\n", str::from_utf8(&write_buffer[..6]).unwrap());
        assert_eq!([0, 0, 0, 0], &write_buffer[6..]);

        // Check history
        let history_vec = history.lock().unwrap();
        assert_eq!(history_vec.len(), 1);
        assert_eq!(history_vec[0], "echo!")
    }

    #[test]
    fn test_echo_lines() {
        let mut write_buffer = [0u8; 15];
        let history = Arc::new(Mutex::from(Vec::new()));

        let result = Connection {
            reader: "haaalo \necho!".as_bytes(),
            writer: &mut write_buffer.as_mut(),
        }
        .echo(Arc::clone(&history));

        assert!(result.is_ok());

        // Check output stream
        assert_eq!(
            "haaalo \necho!\n",
            str::from_utf8(&write_buffer[..14]).unwrap()
        );

        // Check history
        let history_vec = history.lock().unwrap();
        assert_eq!(history_vec.len(), 2);
        assert_eq!(history_vec[0], "haaalo ");
        assert_eq!(history_vec[1], "echo!");
    }
}
