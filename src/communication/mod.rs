pub mod uds;

pub trait Communication {
    fn connect(&mut self) -> std::io::Result<()>;
    fn send(&mut self, data: &[u8]) -> std::io::Result<()>;
    fn recv(&mut self, buffer: &mut [u8]) -> std::io::Result<usize>;
}

pub use uds::UdsServer;
