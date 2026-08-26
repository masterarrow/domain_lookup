use std::sync::Arc;
use axum::{
    routing::get,
    Router,
};
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
    let state = AppState {
        client: Arc::new(client),
    };

    // Axum routes
    let app = Router::new()
        .route("/info", get(domain_info))
        .route("/lookup", get(domain_lookup))
        .route("/whois", get(whois_lookup))
        .route("/ns-lookup", get(ns_lookup))
        .route("/subdomain", get(subdomain_lookup))
        .route("/ssl-check", get(ssl_lookup))
        .route("/health", get(health_check))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await?;
    println!("{}", format!("HTTP Server started on  http://0.0.0.0:{}", port));

    axum::serve(listener, app).await?;

    Ok(())
}
