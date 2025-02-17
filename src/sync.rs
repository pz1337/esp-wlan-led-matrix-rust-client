use std::io::{Read, Write};
use std::net::{TcpStream, ToSocketAddrs};

use bufstream::BufStream;

use crate::Command;

pub struct Client {
    stream: BufStream<TcpStream>,
    width: u8,
    height: u8,
}

impl Client {
    /// Connect to the server
    ///
    /// # Errors
    /// Errors when the connection could not be established.
    pub fn connect(addr: impl ToSocketAddrs) -> std::io::Result<Self> {
        let stream = TcpStream::connect(addr)?;
        let mut stream = BufStream::new(stream);

        let mut buf = [0; 2];
        stream.read_exact(&mut buf)?;

        let width = buf[0];
        let height = buf[1];

        Ok(Self {
            stream,
            width,
            height,
        })
    }

    #[must_use]
    pub fn width(&self) -> u8 {
        self.width
    }

    #[must_use]
    pub fn height(&self) -> u8 {
        self.height
    }

    #[must_use]
    pub fn total_pixels(&self) -> u16 {
        u16::from(self.width) * u16::from(self.height)
    }

    /// Flushes the internal buffer and sends everything to the server
    ///
    /// # Errors
    /// Errors when the command could not be sent
    pub fn flush(&mut self) -> std::io::Result<()> {
        self.stream.flush()
    }

    /// Set one pixel of the matrix to the given color.
    /// Do not forget to also run [flush] afterwards.
    ///
    /// # Errors
    /// Errors when the data could not be written to the send buffer
    ///
    /// [flush]: Self::flush
    pub fn pixel(&mut self, x: u8, y: u8, red: u8, green: u8, blue: u8) -> std::io::Result<()> {
        self.stream
            .write_all(&[Command::Pixel as u8, x, y, red, green, blue])
    }

    /// Fill the whole matrix with one color.
    /// Do not forget to also run [flush] afterwards.
    ///
    /// # Errors
    /// Errors when the command could not be sent
    ///
    /// [flush]: Self::flush
    pub fn fill(&mut self, red: u8, green: u8, blue: u8) -> std::io::Result<()> {
        self.stream
            .write_all(&[Command::Fill as u8, red, green, blue])
    }
}
