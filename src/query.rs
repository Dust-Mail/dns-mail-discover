use std::fmt::Display;

use crate::server::{ServerProtocol, ServerType};

pub struct DnsQuery {
    domain: String,
    server_proto: ServerProtocol,
}

impl Into<ServerProtocol> for DnsQuery {
    fn into(self) -> ServerProtocol {
        self.server_proto
    }
}

impl Display for DnsQuery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "_{}._tcp.{}", self.server_proto, self.domain)
    }
}

impl DnsQuery {
    pub fn new<D: Into<String>>(domain: D, secure: bool, server_type: ServerType) -> Self {
        Self {
            domain: domain.into(),
            server_proto: ServerProtocol::new(server_type, secure),
        }
    }
}
