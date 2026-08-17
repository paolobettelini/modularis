#[cfg(feature = "server")]
mod args;
#[cfg(feature = "server")]
mod config;
#[cfg(feature = "server")]
mod http;
#[cfg(feature = "server")]
mod nats_handler;

#[cfg(feature = "server")]
#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    use std::net::{IpAddr, SocketAddr};
    use std::sync::Arc;
    use std::time::Duration;

    use actix_web::{App as ActixApp, HttpServer, web};
    use clap::Parser;
    use futures_util::StreamExt;
    use leptos::config::{Env, get_config_from_str};
    use leptos::prelude::provide_context;
    use leptos_actix::{LeptosRoutes, generate_route_list};
    use thecrown_common::config::parse_toml_config;
    use thecrown_common::nats::NatsClient;
    use thecrown_database::{Database, DatabaseConfig};
    use thecrown_protocol::{WEB_SUBJECT, WebPacket};
    use thecrown_web::app::{App, shell};
    use thecrown_web::assets::{FrontendAssets, default_dist_root};
    use thecrown_web::state::WebState;

    use crate::args::Args;
    use crate::config::Config;
    use crate::nats_handler::handle_web_packet;

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

    let state = WebState::new(
        database,
        nats.clone(),
        Duration::from_secs(config.web.auth_token_ttl_seconds),
        config.web.session_ttl_seconds,
        config.web.session_cookie_secure,
        config.web.auth_success_redirect.clone(),
    );
    state.start_cleanup_task();

    let nats_state = state.clone();
    let nats_client = nats.clone();
    tokio::spawn(async move {
        let mut subscription = match nats_client.subscribe(WEB_SUBJECT).await {
            Ok(subscription) => subscription,
            Err(error) => {
                log::error!("failed to subscribe to {WEB_SUBJECT}: {error:#}");
                return;
            }
        };
        log::info!("web service listening on NATS subject {WEB_SUBJECT}");

        while let Some(message) = subscription.next().await {
            let packet = match NatsClient::decode::<WebPacket>(&message) {
                Ok(packet) => packet,
                Err(error) => {
                    log::warn!("discarding invalid web packet: {error:#}");
                    continue;
                }
            };
            if let Some(response) = handle_web_packet(&nats_state, packet).await {
                if let Some(reply) = message.reply {
                    if let Err(error) = nats_client.publish(reply, &response).await {
                        log::error!("failed to publish web response: {error:#}");
                    }
                }
            }
        }
    });

    let address: IpAddr = config.web.address.parse()?;
    let listen_addr = SocketAddr::new(address, config.web.port);
    let mut leptos_options = get_config_from_str(include_str!("../Cargo.toml"))?;
    leptos_options.site_addr = listen_addr;
    leptos_options.env = if cfg!(debug_assertions) {
        Env::DEV
    } else {
        Env::PROD
    };

    let routes = generate_route_list(App);
    let web_state = web::Data::new(state);
    let assets = web::Data::new(FrontendAssets::new(default_dist_root()));

    log::info!("TheCrown web listening on http://{listen_addr}");
    if cfg!(debug_assertions) {
        log::info!("frontend assets are read from {}", default_dist_root().display());
    } else {
        log::info!("frontend assets are embedded in the release executable");
    }

    HttpServer::new(move || {
        let context_state = web_state.get_ref().clone();
        let shell_options = leptos_options.clone();

        ActixApp::new()
            .app_data(web_state.clone())
            .app_data(assets.clone())
            .configure(thecrown_web::assets::configure)
            .service(http::health)
            .service(http::auth)
            .service(http::me)
            .service(http::online_players)
            .service(http::player_status)
            .service(http::debug)
            .leptos_routes_with_context(
                routes.clone(),
                move || provide_context(context_state.clone()),
                move || shell(shell_options.clone()),
            )
    })
    .bind(listen_addr)?
    .run()
    .await?;

    Ok(())
}

#[cfg(not(feature = "server"))]
pub fn main() {}
