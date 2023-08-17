use std::fmt::Display;

use tokio::net::ToSocketAddrs;

#[derive(Clone, Copy, Debug)]
pub enum ServerType {
    Smtp,
    Imap,
    Pop,
}

#[derive(Debug)]
pub struct ServerProtocol {
    secure: bool,
    r#type: ServerType,
}

impl ServerProtocol {
    pub fn new(r#type: ServerType, secure: bool) -> Self {
        Self { secure, r#type }
    }

    pub fn secure(&self) -> bool {
        self.secure
    }
}

impl Display for ServerProtocol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.r#type {
            ServerType::Pop => {
                write!(f, "pop3{}", if self.secure { "s" } else { "" })
            }
            ServerType::Imap => {
                write!(f, "imap{}", if self.secure { "s" } else { "" })
            }
            ServerType::Smtp => {
                write!(f, "submission")
            }
        }
    }
}

#[derive(Debug)]
pub struct Server {
    port: u16,
    domain: String,
    proto: ServerProtocol,
}

impl Server {
    pub fn new(port: u16, domain: String, proto: ServerProtocol) -> Self {
        Self {
            port,
            domain,
            proto,
        }
    }

    pub fn socket_addr(&self) -> impl ToSocketAddrs + '_ {
        (self.domain.as_str(), self.port)
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn domain(&self) -> &str {
        self.domain.as_ref()
    }

    pub fn protocol(&self) -> &ServerProtocol {
        &self.proto
    }
}

#[macro_export]
macro_rules! server {
    ($domain:expr, $port:expr, $secure:expr, $server_type:expr) => {{
        use crate::server::{Server, ServerProtocol};

        let proto = ServerProtocol::new($server_type, $secure);

        Server::new($port, $domain, proto)
    }};
}
