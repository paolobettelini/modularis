use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub nats: NatsConfig,
    pub database: DatabaseConfig,
    pub web: WebConfig,
    #[serde(default)]
    pub log: LogConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct NatsConfig {
    pub url: String,
    #[serde(rename = "request-timeout-ms", default = "default_nats_timeout_ms")]
    pub request_timeout_ms: u64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
    #[serde(rename = "max-pool-size", default = "default_pool_size")]
    pub max_pool_size: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WebConfig {
    pub address: String,
    pub port: u16,
    #[serde(rename = "auth-token-ttl-seconds", default = "default_auth_token_ttl")]
    pub auth_token_ttl_seconds: u64,
    #[serde(rename = "session-ttl-seconds", default = "default_session_ttl")]
    pub session_ttl_seconds: i64,
    #[serde(rename = "session-cookie-secure", default)]
    pub session_cookie_secure: bool,
    #[serde(rename = "auth-success-redirect", default = "default_success_redirect")]
    pub auth_success_redirect: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LogConfig {
    #[serde(default = "default_log_filter")]
    pub filter: String,
    #[serde(default = "default_log_style")]
    pub style: String,
}

impl Default for LogConfig {
    fn default() -> Self {
        Self {
            filter: default_log_filter(),
            style: default_log_style(),
        }
    }
}

fn default_nats_timeout_ms() -> u64 { 2_000 }
fn default_pool_size() -> u32 { 15 }
fn default_auth_token_ttl() -> u64 { 300 }
fn default_session_ttl() -> i64 { 2_592_000 }
fn default_success_redirect() -> String { "/".to_owned() }
fn default_log_filter() -> String { "info".to_owned() }
fn default_log_style() -> String { "auto".to_owned() }
