pub mod config;
pub mod client;

use std::{collections::HashMap, sync::Arc};
use axum::{
    http::StatusCode,
    extract::{Query, State},
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use reqwest::RequestBuilder;
use validator::{Validate, ValidationError};
use regex::Regex;

pub use config::Config;
pub use client::HttpClient;

#[derive(Clone)]
pub struct AppState {
    pub client: Arc<HttpClient>,
}

#[derive(Deserialize, Serialize, Validate)]
pub struct RequestParams {
    #[validate(custom(function = "validate_domain"))]
    domain: String,
}

fn validate_domain(domain: &str) -> Result<(), ValidationError> {
    let domain_regex = Regex::new(
        r"^(?i)(?:[a-z0-9](?:[a-z0-9-]{0,61}[a-z0-9])?\.)+[a-z]{2,}$"
    ).unwrap();

    if !domain_regex.is_match(domain) {
        let e = ValidationError {
            code: "domain".into(),
            message: Some("Invalid domain name provided.".into()),
            params: HashMap::new(),
        };
        return Err(e);
    }

    Ok(())
}

pub async fn domain_info(
    State(state): State<AppState>,
    Query(params): Query<RequestParams>,
) -> Result<impl IntoResponse, StatusCode> {
    if let Err(errors) = params.validate() {
        return serialize_response(
            ("result".to_string(), json!({"errors": errors.errors().get("domain")})),
            StatusCode::BAD_REQUEST
        );
    }

    let domain = params.domain;

    let whois_req = lookup(state.client.whois_lookup(&domain));
    let ns_req = lookup(state.client.ns_lookup(&domain));
    let sub_req = lookup(state.client.subdomain_lookup(&domain));

    let (whois_resp, ns_resp, sub_resp) = tokio::join!(whois_req, ns_req, sub_req);

    let (whois_data, ns_data, sub_data) = match (whois_resp, ns_resp, sub_resp) {
        (Ok(whois_data), Ok(dns_data), Ok(sub_data)) => (whois_data, dns_data, sub_data),
        (Err(e), _, _) => return Err(e),
        (_, Err(e), _) => return Err(e),
        (_, _, Err(e),) => return Err(e),
    };

    let domain = json!({
        "whois": whois_data,
        "ns": ns_data,
        "subdomains": sub_data
    });

    serialize_response(("result".to_string(), domain), StatusCode::OK)
}

pub async fn health_check() -> impl IntoResponse {
    Json(json!({"status": "OK"}))
}

pub async fn domain_lookup(
    State(state): State<AppState>,
    Query(params): Query<RequestParams>,
) -> Result<impl IntoResponse, StatusCode> {
    if let Err(errors) = params.validate() {
        return serialize_response(
            ("result".to_string(), json!({"errors": errors.errors().get("domain")})),
            StatusCode::BAD_REQUEST
        );
    }
    let lookup_data = lookup(state.client.domain_lookup(&params.domain)).await?;

    serialize_response(("result".to_string(), lookup_data), StatusCode::OK)
}

pub async fn whois_lookup(
    State(state): State<AppState>,
    Query(params): Query<RequestParams>,
) -> Result<impl IntoResponse, StatusCode> {
    if let Err(errors) = params.validate() {
        return serialize_response(
            ("result".to_string(), json!({"errors": errors.errors().get("domain")})),
            StatusCode::BAD_REQUEST
        );
    }
    let lookup_data = lookup(state.client.whois_lookup(&params.domain)).await?;

    serialize_response(("result".to_string(), lookup_data), StatusCode::OK)
}

pub async fn ns_lookup(
    State(state): State<AppState>,
    Query(params): Query<RequestParams>,
) -> Result<impl IntoResponse, StatusCode> {
    if let Err(errors) = params.validate() {
        return serialize_response(
            ("result".to_string(), json!({"errors": errors.errors().get("domain")})),
            StatusCode::BAD_REQUEST
        );
    }
    let lookup_data = lookup(state.client.ns_lookup(&params.domain)).await?;

    serialize_response(("result".to_string(), lookup_data), StatusCode::OK)
}

pub async fn subdomain_lookup(
    State(state): State<AppState>,
    Query(params): Query<RequestParams>,
) -> Result<impl IntoResponse, StatusCode> {
    if let Err(errors) = params.validate() {
        return serialize_response(
            ("result".to_string(), json!({"errors": errors.errors().get("domain")})),
            StatusCode::BAD_REQUEST
        );
    }
    let lookup_data = lookup(state.client.subdomain_lookup(&params.domain)).await?;

    serialize_response(("result".to_string(), lookup_data), StatusCode::OK)
}

pub async fn ssl_lookup(
    State(state): State<AppState>,
    Query(params): Query<RequestParams>,
) -> Result<impl IntoResponse, StatusCode> {
    if let Err(errors) = params.validate() {
        return serialize_response(
            ("result".to_string(), json!({"errors": errors.errors().get("domain")})),
            StatusCode::BAD_REQUEST
        );
    }
    let lookup_data = lookup(state.client.ssl_lookup(&params.domain)).await?;

    serialize_response(("result".to_string(), lookup_data), StatusCode::OK)
}

async fn lookup(builder: RequestBuilder) -> Result<serde_json::Value, StatusCode> {
    let lookup_req = builder.send();

    let lookup_data: serde_json::Value = lookup_req
        .await
        .map_err(|e| {
            eprintln!("Lookup request failed: {e}");
            StatusCode::BAD_GATEWAY
        })?
        .json()
        .await
        .map_err(|e| {
            eprintln!("Failed to parse lookup JSON: {e}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(lookup_data)
}

fn serialize_response(data: (String, serde_json::Value), code: StatusCode) -> Result<impl IntoResponse, StatusCode> {
    let (key, val) = data;

    let res = json!({
        "code": code.as_u16(),
        "success": code == StatusCode::OK,
        key: val,
    });

    Ok((code, Json(res)))
}
