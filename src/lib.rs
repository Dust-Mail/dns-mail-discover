use client::Client;
use error::Result;
use server::Server;

use crate::error::ErrorKind;

mod client;
mod dns;
pub mod error;
mod query;
pub mod server;
// mod tcp;

pub async fn from_domain<D: AsRef<str>>(domain: D) -> Result<Vec<Server>> {
    let client = Client::new()?;

    // let mut servers = Vec::new();

    let servers = client.dns_lookup(domain).await;

    if servers.is_empty() {
        failed!(
            ErrorKind::NotFound,
            "Could not find any servers from the given domain"
        )
    }

    Ok(servers)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[tokio::test]
    async fn test_from_domain() {
        let servers = from_domain("gmail.com").await.unwrap();

        assert_eq!(servers.len(), 3);
    }
}
