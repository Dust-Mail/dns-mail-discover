# Dns mail discover

This crate is able to take any domain (stripped from an email address) and find its corresponding email servers using DNS-based SRV lookup. It also tries some basic heuristics, such as `mail.{domain}` or `imap.{domain}` and checks if there are any functioning mail servers.
