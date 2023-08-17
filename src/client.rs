use log::warn;

use crate::{
    dns::Dns,
    error::Result,
    query::DnsQuery,
    server::{Server, ServerType::*},
};

pub struct Client {
    dns: Dns,
}

impl Client {
    pub fn new() -> Result<Self> {
        let dns = Dns::new()?;

        let client = Self { dns };

        Ok(client)
    }

    /// Lookup basic dns settings to find mail servers according to https://datatracker.ietf.org/doc/html/rfc6186
    pub async fn dns_lookup<D: AsRef<str>>(&self, domain: D) -> Vec<Server> {
        let server_types = vec![Pop, Imap];

        let mut queries = Vec::new();

        for server_type in server_types {
            let plain = DnsQuery::new(domain.as_ref(), false, server_type);
            let secure = DnsQuery::new(domain.as_ref(), true, server_type);

            queries.push(plain);
            queries.push(secure);
        }

        queries.push(DnsQuery::new(domain.as_ref(), true, Smtp));

        let mut servers = Vec::new();

        for query in queries {
            let result = self.dns.srv_lookup(&query).await;

            match result {
                Ok((domain, port)) => {
                    if &domain != "." {
                        let server = Server::new(port, domain, query.into());
                        servers.push(server);
                    }
                }
                Err(error) => {
                    warn!("SRV lookup failed for {}: {}", query, error)
                }
            }
        }

        servers
    }

    // pub async fn from_heuristics<D: AsRef<str>>(&self, domain: D) {
    //     let to_check = vec![
    //         server!(format!("mail.{}", domain.as_ref()), 143, false, Imap),
    //         server!(format!("imap.{}", domain.as_ref()), 143, false, Imap),
    //         server!(format!("mail.{}", domain.as_ref()), 993, true, Imap),
    //         server!(format!("imap.{}", domain.as_ref()), 993, true, Imap),
    //     ];
    // }
}
