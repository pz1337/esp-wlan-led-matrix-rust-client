use tokio::io::{AsyncReadExt, AsyncWriteExt, BufStream};
use tokio::net::{TcpStream, ToSocketAddrs};

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
    pub async fn connect(addr: impl ToSocketAddrs) -> std::io::Result<Self> {
        let stream = TcpStream::connect(addr).await?;
        let mut stream = BufStream::new(stream);

        let width = {
            let mut buf = [0; 1];
            if 1 != stream.read(&mut buf).await? {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "failed to receive width",
                ));
            }
            buf[0]
        };

        let height = {
            let mut buf = [0; 1];
            if 1 != stream.read(&mut buf).await? {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "failed to receive height",
                ));
            }
            buf[0]
        };

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
    pub async fn flush(&mut self) -> std::io::Result<()> {
        self.stream.flush().await
    }

    /// Set one pixel of the matrix to the given color.
    /// Do not forget to also run [flush] afterwards.
    ///
    /// # Errors
    /// Errors when the data could not be written to the send buffer
    ///
    /// [flush]: Self::flush
    pub async fn pixel(
        &mut self,
        x: u8,
        y: u8,
        red: u8,
        green: u8,
        blue: u8,
    ) -> std::io::Result<()> {
        self.stream
            .write_all(&[Command::Pixel as u8, x, y, red, green, blue])
            .await
    }

    /// Fill the whole matrix with one color.
    /// Do not forget to also run [flush] afterwards.
    ///
    /// # Errors
    /// Errors when the command could not be sent
    ///
    /// [flush]: Self::flush
    pub async fn fill(&mut self, red: u8, green: u8, blue: u8) -> std::io::Result<()> {
        self.stream
            .write_all(&[Command::Fill as u8, red, green, blue])
            .await
    }
}
