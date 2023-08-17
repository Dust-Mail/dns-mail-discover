use std::time::Duration;

use tokio::io::{AsyncBufReadExt, AsyncRead, AsyncWrite, AsyncWriteExt, BufStream};

use crate::{
    error::{ErrorKind, Result},
    failed,
};

// \n
const LF: u8 = 0x0a;
// \r
const CR: u8 = 0x0d;

const TIMEOUT: Duration = Duration::from_secs(5);

pub struct TcpClient<S: AsyncWrite + AsyncRead + Unpin> {
    stream: BufStream<S>,
}

impl<S: AsyncWrite + AsyncRead + Unpin> From<S> for TcpClient<S> {
    fn from(stream: S) -> Self {
        Self {
            stream: BufStream::new(stream),
        }
    }
}

impl<S: AsyncWrite + AsyncRead + Unpin> TcpClient<S> {
    /// Expects a single line response
    pub async fn send_command<C: AsRef<[u8]>>(&mut self, command: C) -> Result<String> {
        self.send_bytes(command).await?;

        let response = self.read_response().await?;

        Ok(response)
    }

    /// Write some bytes to the socket, end it off with a LF CR and send it to the remote server.
    pub async fn send_bytes<B: AsRef<[u8]>>(&mut self, bytes: B) -> Result<()> {
        self.stream.write_all(bytes.as_ref()).await?;
        self.stream.write_all(&[LF, CR]).await?;
        self.stream.flush().await?;

        Ok(())
    }

    /// Reads a single line response
    pub async fn read_response(&mut self) -> Result<String> {
        let mut response = String::new();

        let bytes_read = self.stream.read_line(&mut response).await?;

        if bytes_read < 1 {
            failed!(
                ErrorKind::NoBytesSent,
                "The remote server did not send any bytes"
            );
        }

        Ok(response)
    }

    pub async fn close(&mut self) -> Result<()> {
        self.stream.shutdown().await?;

        Ok(())
    }
}
