use std::net::Ipv4Addr;
use url::Url;

/// Validate format host for domain and IPv4
pub fn is_valid_host(url: &Url) -> bool {
    let host = match url.host_str() {
        Some(h) => h,
        None => return false,
    };

    if host.parse::<Ipv4Addr>().is_ok() {
        return true;
    }

    if host.starts_with('.') || host.ends_with('.') {
        return false;
    }

    let parts: Vec<&str> = host.split('.').collect();

    if parts.len() < 2 {
        return false;
    }

    for part in parts {
        if part.is_empty() {
            return false;
        }
        if !part.chars().all(|c| c.is_ascii_alphanumeric() || c == '-') {
            return false;
        }
    }

    true
}
