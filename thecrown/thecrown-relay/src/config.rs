use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub nats: NatsConfig,
    pub database: DatabaseConfig,
    pub relay: RelayConfig,
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
pub struct RelayConfig {
    #[serde(rename = "auth-ticket-ttl-seconds", default = "default_auth_ticket_ttl")]
    pub auth_ticket_ttl_seconds: u64,
    #[serde(
        rename = "initial-instances-per-mode",
        default = "default_initial_instances_per_mode"
    )]
    pub initial_instances_per_mode: usize,
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
fn default_auth_ticket_ttl() -> u64 { 10 }
fn default_initial_instances_per_mode() -> usize { 2 }
fn default_log_filter() -> String { "info".to_owned() }
fn default_log_style() -> String { "auto".to_owned() }
