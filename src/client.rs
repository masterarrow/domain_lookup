use reqwest::{Client, RequestBuilder, header::{ACCEPT, AUTHORIZATION, HeaderMap, HeaderValue}};

use crate::Config;

pub struct HttpClient {
    client: Client,
    conf: Config,
}

impl HttpClient {
    pub fn new(conf: Config) -> Self {
        let client = Client::builder()
            .brotli(true)
            .gzip(true)
            .default_headers(Self::headers(&conf.access_token))
            .build()
            .expect("Failed to build HTTP client");

        HttpClient {
            client,
            conf
        }
    }

    fn headers(token: &str) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(ACCEPT, HeaderValue::from_static("application/json"));

        let auth_value = HeaderValue::from_str(&format!("TOKEN={}", token))
            .expect("Invalid characters in ACCESS_TOKEN");

        headers.insert(AUTHORIZATION, auth_value);

        headers
    }

    fn get(&self, url: &str) -> RequestBuilder {
        self.client.get(url)
    }

    fn format_url(&self, url: &str, domain: &str) -> String {
        format!("{}/{}/{}?domain={}", self.conf.api_url, self.conf.api_version, url, domain)
    }

    pub fn domain_lookup(&self, domain: &str) -> RequestBuilder {
        let url = self.format_url("domain-availability", domain);

        self.get(&url)
    }

    pub fn whois_lookup(&self, domain: &str) -> RequestBuilder {
        let url = self.format_url("whois/", domain);

        self.get(&url)
    }

    pub fn ns_lookup(&self, domain: &str) -> RequestBuilder {
        let url = self.format_url("nslookup", domain);

        self.get(&url)
    }

    pub fn subdomain_lookup(&self, domain: &str) -> RequestBuilder {
        let url = self.format_url("subdomains", domain);

        self.get(&url)
    }

    pub fn ssl_lookup(&self, domain: &str) -> RequestBuilder {
        let url = self.format_url("ssl-cert-check", domain);

        self.get(&url)
    }
}
