mod args;
mod config;
mod handler;
mod state;

use std::sync::Arc;
use std::time::Duration;

use clap::Parser;
use futures_util::StreamExt;
use thecrown_common::config::parse_toml_config;
use thecrown_common::nats::NatsClient;
use thecrown_database::{Database, DatabaseConfig};
use thecrown_protocol::{RELAY_SUBJECT, RelayPacket};

use crate::args::Args;
use crate::config::Config;
use crate::handler::handle_msg;
use crate::state::State;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let config: Config = parse_toml_config(&args.config)?;

    let env = env_logger::Env::default()
        .filter_or("RUST_LOG", config.log.filter.clone())
        .write_style_or("RUST_LOG_STYLE", config.log.style.clone());
    env_logger::init_from_env(env);

    let mut db_config = DatabaseConfig::new(config.database.url.clone());
    db_config.max_pool_size = config.database.max_pool_size;
    let database = Arc::new(Database::connect_with_config(db_config)?);
    let nats = Arc::new(
        NatsClient::connect(
            &config.nats.url,
            Duration::from_millis(config.nats.request_timeout_ms),
        )
        .await?,
    );

    let state = State::new(
        nats.clone(),
        database,
        Duration::from_secs(config.relay.auth_ticket_ttl_seconds),
        config.relay.initial_instances_per_mode,
    );
    state.start_cleanup_task();

    let mut subscription = nats.subscribe(RELAY_SUBJECT).await?;
    log::info!("relay listening on NATS subject {RELAY_SUBJECT}");

    while let Some(message) = subscription.next().await {
        let packet = match NatsClient::decode::<RelayPacket>(&message) {
            Ok(packet) => packet,
            Err(error) => {
                log::warn!("discarding invalid relay packet: {error:#}");
                continue;
            }
        };

        if let Some(response) = handle_msg(&state, packet).await {
            if let Some(reply) = message.reply {
                if let Err(error) = nats.publish(reply, &response).await {
                    log::error!("failed to publish relay response: {error:#}");
                }
            } else {
                log::debug!("response packet dropped because request had no reply subject");
            }
        }
    }

    anyhow::bail!("NATS relay subscription ended")
}
