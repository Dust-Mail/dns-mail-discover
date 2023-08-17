use async_native_tls::TlsConnector;
use log::warn;
use tokio::{
    io::{AsyncRead, AsyncWrite},
    net::TcpStream,
};

use crate::{
    error::Result,
    server::{
        Server,
        ServerType::{self, *},
    },
};

use self::client::TcpClient;

mod client;

pub struct MailServerTypeDetector;

impl MailServerTypeDetector {
    async fn check_for_imap<S: AsyncRead + AsyncWrite + Unpin>(
        client: &mut TcpClient<S>,
    ) -> Result<bool> {
        let response = client.send_command("A0001 CAPABILITY").await?;

        let response = response.trim().to_ascii_lowercase();

        let is_imap = response
            .split(' ')
            .find(|capability| capability == &"imap4rev1")
            .is_some();

        Ok(is_imap)
    }

    async fn check_for_smtp<R: AsRef<str>>(greeting: R) -> Result<bool> {
        let greeting = greeting.as_ref().trim().to_ascii_lowercase();

        let is_smtp = greeting.contains("smtp") || greeting.contains("esmtp");

        Ok(is_smtp)
    }

    pub async fn detect<S: AsyncRead + AsyncWrite + Unpin, T: Into<TcpClient<S>>>(
        socket: T,
    ) -> Result<Option<ServerType>> {
        let mut client: TcpClient<S> = socket.into();

        let greeting = client.read_response().await?;

        match Self::check_for_imap(&mut client).await {
            Ok(is_imap) => {
                if is_imap {
                    return Ok(Some(Imap));
                }
            }
            Err(err) => {
                warn!("Check for IMAP failed: {}", err)
            }
        }

        match Self::check_for_smtp(&greeting).await {
            Ok(is_smtp) => {
                if is_smtp {
                    return Ok(Some(Smtp));
                }
            }
            Err(err) => {
                warn!("Check for SMTP failed: {}", err)
            }
        }

        Ok(None)
    }

    pub async fn detect_from_server(server: Server) -> Result<Option<ServerType>> {
        let socket = TcpStream::connect(server.socket_addr()).await?;

        let server_type = if server.protocol().secure() {
            let tls = TlsConnector::new();

            let tls_socket = tls.connect(server.domain(), socket).await?;

            Self::detect(tls_socket).await?
        } else {
            Self::detect(socket).await?
        };

        Ok(server_type)
    }
}
