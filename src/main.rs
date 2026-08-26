use std::{sync::Arc, time::Duration};
use axum::{
    routing::get,
    Router,
};
use std::net::SocketAddr;
use tower_governor::{governor::GovernorConfigBuilder, GovernorLayer};
use domain_lookup::{
    AppState,
    Config,
    HttpClient,
    domain_info,
    domain_lookup,
    whois_lookup,
    ns_lookup,
    subdomain_lookup,
    ssl_lookup,
    health_check,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let config = Config::new();
    let port = config.port;
    let client = HttpClient::new(config);

    // Rate limit 60 requests per minute with a burst size of 20
    let governor_conf = Arc::new(
        GovernorConfigBuilder::default()
            .per_second(1)
            .burst_size(20)
            .finish()
            .unwrap(),
    );
    let governor_limiter = governor_conf.limiter().clone();
    let interval = Duration::from_secs(60);

    let state = AppState {
        client: Arc::new(client),
    };

    std::thread::spawn(move || {
       loop {
           std::thread::sleep(interval);
           governor_limiter.retain_recent();
       }
   });

    // Axum routes
    let app = Router::new()
        .route("/info", get(domain_info))
        .route("/lookup", get(domain_lookup))
        .route("/whois", get(whois_lookup))
        .route("/ns-lookup", get(ns_lookup))
        .route("/subdomain", get(subdomain_lookup))
        .route("/ssl-check", get(ssl_lookup))
        .route("/health", get(health_check))
        .layer(GovernorLayer::new(governor_conf))
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = tokio::net::TcpListener::bind(addr).await?;
    println!("{}", format!("HTTP Server started on  http://0.0.0.0:{}", port));

    axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>()).await?;

    Ok(())
}
