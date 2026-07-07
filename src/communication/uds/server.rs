use std::fs;
use std::io::{self, Read, Write};
use std::os::unix::net::{UnixListener, UnixStream};

use crate::communication::Communication;

pub struct UdsServer {
    socket_path: String,
    listener: Option<UnixListener>,
    stream: Option<UnixStream>,
}

impl UdsServer {
    pub fn new(path: &str) -> Self {
        Self {
            socket_path: path.to_string(),
            listener: None,
            stream: None,
        }
    }
}

impl Communication for UdsServer {
    fn connect(&mut self) -> io::Result<()> {
        let _ = fs::remove_file(&self.socket_path);

        let listener = UnixListener::bind(&self.socket_path)?;

        println!("Waiting for client...");

        let (stream, _) = listener.accept()?;

        println!("Client connected");

        self.listener = Some(listener);
        self.stream = Some(stream);

        Ok(())
    }

    fn send(&mut self, data: &[u8]) -> io::Result<()> {
        if let Some(stream) = self.stream.as_mut() {
            stream.write_all(data)?;
        }

        Ok(())
    }

    fn recv(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
        if let Some(stream) = self.stream.as_mut() {
            return stream.read(buffer);
        }

        Ok(0)
    }
}
