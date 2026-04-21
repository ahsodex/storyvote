mod http;
mod messages;
mod state;
mod ui;
mod ws;

use clap::Parser;
use dns_lookup::{lookup_addr, lookup_host};
use local_ip_address::local_ip;
use state::SharedState;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::Arc;
use tokio::net::TcpListener;
use tracing::{info, warn};

#[derive(Debug, Parser)]
#[command(name = "storyvote")]
#[command(about = "Ephemeral agile story point estimation server")]
struct Args {
    #[arg(long)]
    bind: Option<IpAddr>,

    #[arg(long, default_value_t = 8787)]
    port: u16,

    #[arg(long, default_value_t = false)]
    random_port: bool,

    #[arg(long, default_value_t = false)]
    localhost_only: bool,

    #[arg(long, default_value_t = false)]
    http_access_log: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "storyvote=info,axum=info".to_string()),
        )
        .init();

    let args = Args::parse();
    let bind_ip = resolve_bind_ip(&args);
    let bind_port = if args.random_port { 0 } else { args.port };

    let listener = TcpListener::bind(SocketAddr::new(bind_ip, bind_port)).await?;
    let local_addr = listener.local_addr()?;

    let shared = Arc::new(SharedState::new());
    let app = http::router(shared, args.http_access_log);

    print_share_urls(local_addr, args.localhost_only);

    info!("Server listening on {}", local_addr);
    axum::serve(listener, app).await?;
    Ok(())
}

fn resolve_bind_ip(args: &Args) -> IpAddr {
    if let Some(bind) = args.bind {
        return bind;
    }

    if args.localhost_only {
        return IpAddr::V4(Ipv4Addr::LOCALHOST);
    }

    IpAddr::V4(Ipv4Addr::UNSPECIFIED)
}

fn print_share_urls(local_addr: SocketAddr, localhost_only: bool) {
    let detected_ip = local_ip().ok();
    let detected_fqdn = detected_ip.and_then(detect_fqdn_for_ip);

    let (share_url, local_url) =
        match build_share_urls(local_addr, localhost_only, detected_ip, detected_fqdn.as_deref()) {
        Ok(urls) => urls,
        Err(error) => {
            warn!("Share URL fallback used: {error}");
            let localhost_url = format!("http://localhost:{}", local_addr.port());
            (localhost_url.clone(), Some(localhost_url))
        }
    };

    println!("StoryVote ready");
    println!("Share URL: {share_url}");
    if let Some(local_url) = local_url {
        if local_url != share_url {
            println!("Local URL: {local_url}");
        }
    }
}

fn build_share_urls(
    local_addr: SocketAddr,
    localhost_only: bool,
    detected_ip: Option<IpAddr>,
    detected_fqdn: Option<&str>,
) -> Result<(String, Option<String>), &'static str> {
    let port = local_addr.port();
    let localhost_url = format!("http://localhost:{port}");

    if localhost_only {
        return Ok((localhost_url, None));
    }

    if let Some(host) = detected_fqdn {
        return Ok((format!("http://{host}:{port}"), Some(localhost_url)));
    }

    let ip = detected_ip.ok_or("local IP unavailable")?;
    Ok((format!("http://{ip}:{port}"), Some(localhost_url)))
}

fn detect_fqdn_for_ip(ip: IpAddr) -> Option<String> {
    let raw_host = lookup_addr(&ip).ok()?;
    let hostname = raw_host.trim_end_matches('.').to_string();

    if !hostname.contains('.') {
        return None;
    }

    let forward_matches = lookup_host(&hostname)
        .ok()
        .map(|resolved| resolved.into_iter().any(|resolved_ip| resolved_ip == ip))
        .unwrap_or(false);

    if forward_matches {
        Some(hostname)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::{build_share_urls, resolve_bind_ip, Args};
    use std::net::{IpAddr, Ipv4Addr, SocketAddr};

    #[test]
    fn localhost_only_uses_loopback_bind() {
        let args = Args {
            bind: None,
            port: 8787,
            random_port: false,
            localhost_only: true,
            http_access_log: false,
        };

        assert_eq!(resolve_bind_ip(&args), IpAddr::V4(Ipv4Addr::LOCALHOST));
    }

    #[test]
    fn explicit_bind_takes_precedence() {
        let args = Args {
            bind: Some(IpAddr::V4(Ipv4Addr::new(192, 168, 1, 50))),
            port: 8787,
            random_port: false,
            localhost_only: true,
            http_access_log: false,
        };

        assert_eq!(
            resolve_bind_ip(&args),
            IpAddr::V4(Ipv4Addr::new(192, 168, 1, 50))
        );
    }

    #[test]
    fn build_share_urls_returns_lan_and_localhost_urls() {
        let urls = build_share_urls(
            SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), 8787),
            false,
            Some(IpAddr::V4(Ipv4Addr::new(10, 0, 0, 5))),
            None,
        )
        .unwrap();

        assert_eq!(urls.0, "http://10.0.0.5:8787");
        assert_eq!(urls.1.as_deref(), Some("http://localhost:8787"));
    }

    #[test]
    fn build_share_urls_returns_localhost_only_when_requested() {
        let urls = build_share_urls(
            SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 8787),
            true,
            None,
            None,
        )
        .unwrap();

        assert_eq!(urls.0, "http://localhost:8787");
        assert_eq!(urls.1, None);
    }

    #[test]
    fn build_share_urls_prefers_detected_fqdn_when_available() {
        let urls = build_share_urls(
            SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), 8787),
            false,
            Some(IpAddr::V4(Ipv4Addr::new(10, 0, 0, 5))),
            Some("storyvote.example.internal"),
        )
        .unwrap();

        assert_eq!(urls.0, "http://storyvote.example.internal:8787");
        assert_eq!(urls.1.as_deref(), Some("http://localhost:8787"));
    }
}
