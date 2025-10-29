use std::net::Ipv4Addr;
use url::Url;

pub fn normalize_url(raw: &str) -> Result<Url, &'static str> {
    // Try normal parse
    let parsed = Url::parse(raw).or_else(|_| Url::parse(&format!("https://{}", raw)));

    let url = parsed.map_err(|_| "must be a valid url")?;

    if !is_valid_host(&url) {
        return Err("must be a valid url");
    }

    Ok(url)
}

/// Validate format host for domain and IPv4
fn is_valid_host(url: &Url) -> bool {
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
