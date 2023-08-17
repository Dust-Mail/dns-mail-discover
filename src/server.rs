use std::fmt::Display;

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
                write!(f, "smtp{}", if self.secure { "s" } else { "" })
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
