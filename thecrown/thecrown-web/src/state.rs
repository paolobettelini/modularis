use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

use thecrown_common::nats::NatsClient;
use thecrown_common::token::random_token;
use thecrown_database::Database;
use tokio::sync::Mutex;
use tokio::time::Instant;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct WebState {
    pub db: Arc<Database>,
    pub nats: Arc<NatsClient>,
    pending_auth: Arc<Mutex<HashMap<String, PendingWebAuth>>>,
    pub auth_token_ttl: Duration,
    pub session_ttl_seconds: i64,
    pub session_cookie_secure: bool,
    pub auth_success_redirect: String,
}

#[derive(Debug, Clone)]
struct PendingWebAuth {
    player_uuid: Uuid,
    expires_at: Instant,
}

impl WebState {
    pub fn new(
        db: Arc<Database>,
        nats: Arc<NatsClient>,
        auth_token_ttl: Duration,
        session_ttl_seconds: i64,
        session_cookie_secure: bool,
        auth_success_redirect: String,
    ) -> Self {
        Self {
            db,
            nats,
            pending_auth: Default::default(),
            auth_token_ttl,
            session_ttl_seconds,
            session_cookie_secure,
            auth_success_redirect,
        }
    }

    pub fn start_cleanup_task(&self) {
        let state = self.clone();
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(30));
            loop {
                interval.tick().await;
                let now = Instant::now();
                state
                    .pending_auth
                    .lock()
                    .await
                    .retain(|_, auth| auth.expires_at > now);
            }
        });

        let db = self.db.clone();
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(300));
            loop {
                interval.tick().await;
                let db = db.clone();
                match tokio::task::spawn_blocking(move || db.purge_expired_user_tokens()).await {
                    Ok(Ok(_)) => {}
                    Ok(Err(error)) => log::warn!("failed to purge expired web sessions: {error}"),
                    Err(error) => log::warn!("web-session cleanup task failed: {error}"),
                }
            }
        });
    }

    pub async fn generate_auth_token(&self, player_uuid: Uuid) -> String {
        let token = random_token();
        self.pending_auth.lock().await.insert(
            token.clone(),
            PendingWebAuth {
                player_uuid,
                expires_at: Instant::now() + self.auth_token_ttl,
            },
        );
        token
    }

    pub async fn consume_auth_token(&self, token: &str) -> Option<Uuid> {
        let auth = self.pending_auth.lock().await.remove(token)?;
        (auth.expires_at > Instant::now()).then_some(auth.player_uuid)
    }
}
