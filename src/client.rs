use log::warn;

use crate::{
    dns::Dns,
    error::Result,
    query::Query,
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

    pub async fn dns_lookup<D: AsRef<str>>(&self, domain: D) -> Vec<Server> {
        let server_types = vec![Pop, Smtp, Imap];

        let mut queries = Vec::new();

        for server_type in server_types {
            let plain = Query::new(domain.as_ref(), false, server_type);
            let secure = Query::new(domain.as_ref(), true, server_type);

            queries.push(plain);
            queries.push(secure);
        }

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
}
