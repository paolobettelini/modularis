use actix_web::cookie::{Cookie, SameSite};
use actix_web::{HttpRequest, HttpResponse, Responder, get, web};
use serde::Serialize;
use thecrown_common::token::random_token;
use thecrown_protocol::{PlayerStatus, RELAY_SUBJECT, RelayPacket};
use uuid::Uuid;

use thecrown_web::state::WebState;

#[derive(Debug, Serialize)]
struct HealthResponse {
    status: &'static str,
}

#[derive(Debug, Serialize)]
struct OnlinePlayersResponse {
    count: u64,
}

#[derive(Debug, Serialize)]
struct PlayerStatusResponse {
    player_uuid: Uuid,
    status: PlayerStatus,
}

async fn query_player_status(state: &WebState, player_uuid: Uuid) -> HttpResponse {
    match state
        .nats
        .request::<_, RelayPacket, _>(
            RELAY_SUBJECT,
            &RelayPacket::GetPlayerStatus { player_uuid },
        )
        .await
    {
        Ok(RelayPacket::ServePlayerStatus { status }) => {
            HttpResponse::Ok().json(PlayerStatusResponse { player_uuid, status })
        }
        Ok(_) => HttpResponse::BadGateway().body("relay returned the wrong packet type"),
        Err(error) => {
            log::warn!("relay player-status query failed: {error:#}");
            HttpResponse::ServiceUnavailable().finish()
        }
    }
}

#[get("/health")]
pub async fn health() -> impl Responder {
    web::Json(HealthResponse { status: "ok" })
}

#[get("/auth/{token}")]
pub async fn auth(token: web::Path<String>, state: web::Data<WebState>) -> impl Responder {
    let Some(player_uuid) = state.consume_auth_token(token.as_str()).await else {
        return HttpResponse::Unauthorized().finish();
    };

    let session_token = random_token();
    let db = state.db.clone();
    let token_for_db = session_token.clone();
    let ttl = state.session_ttl_seconds;
    let stored = tokio::task::spawn_blocking(move || {
        db.add_user_token(player_uuid, &token_for_db, ttl)
    })
    .await;

    match stored {
        Ok(Ok(())) => {}
        Ok(Err(error)) => {
            log::error!("failed to store web session: {error}");
            return HttpResponse::InternalServerError().finish();
        }
        Err(error) => {
            log::error!("web session database task failed: {error}");
            return HttpResponse::InternalServerError().finish();
        }
    }

    let mut uuid_cookie = Cookie::new("player_uuid", player_uuid.hyphenated().to_string());
    uuid_cookie.set_path("/");
    uuid_cookie.set_http_only(true);
    uuid_cookie.set_same_site(SameSite::Lax);
    uuid_cookie.set_secure(state.session_cookie_secure);

    let mut token_cookie = Cookie::new("token", session_token);
    token_cookie.set_path("/");
    token_cookie.set_http_only(true);
    token_cookie.set_same_site(SameSite::Lax);
    token_cookie.set_secure(state.session_cookie_secure);

    HttpResponse::Found()
        .append_header(("Location", state.auth_success_redirect.as_str()))
        .cookie(uuid_cookie)
        .cookie(token_cookie)
        .finish()
}

#[get("/api/online-players")]
pub async fn online_players(state: web::Data<WebState>) -> impl Responder {
    match state
        .nats
        .request::<_, RelayPacket, _>(RELAY_SUBJECT, &RelayPacket::GetOnlinePlayers)
        .await
    {
        Ok(RelayPacket::ServeOnlinePlayers { count }) => {
            HttpResponse::Ok().json(OnlinePlayersResponse { count })
        }
        Ok(_) => HttpResponse::BadGateway().body("relay returned the wrong packet type"),
        Err(error) => {
            log::warn!("relay online-player query failed: {error:#}");
            HttpResponse::ServiceUnavailable().finish()
        }
    }
}

#[get("/api/player/{uuid}")]
pub async fn player_status(path: web::Path<String>, state: web::Data<WebState>) -> impl Responder {
    let player_uuid = match Uuid::parse_str(path.as_str()) {
        Ok(uuid) => uuid,
        Err(_) => return HttpResponse::BadRequest().body("invalid UUID"),
    };
    query_player_status(state.get_ref(), player_uuid).await
}

#[get("/api/me")]
pub async fn me(request: HttpRequest, state: web::Data<WebState>) -> impl Responder {
    let Some(uuid_cookie) = request.cookie("player_uuid") else {
        return HttpResponse::Unauthorized().finish();
    };
    let Some(token_cookie) = request.cookie("token") else {
        return HttpResponse::Unauthorized().finish();
    };
    let player_uuid = match Uuid::parse_str(uuid_cookie.value()) {
        Ok(uuid) => uuid,
        Err(_) => return HttpResponse::Unauthorized().finish(),
    };
    let token = token_cookie.value().to_owned();
    let db = state.db.clone();
    let authenticated = tokio::task::spawn_blocking(move || db.user_has_token(player_uuid, &token))
        .await;
    match authenticated {
        Ok(Ok(true)) => query_player_status(state.get_ref(), player_uuid).await,
        Ok(Ok(false)) => HttpResponse::Unauthorized().finish(),
        Ok(Err(error)) => {
            log::error!("failed to validate web session: {error}");
            HttpResponse::InternalServerError().finish()
        }
        Err(error) => {
            log::error!("web session validation task failed: {error}");
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[get("/api/debug")]
pub async fn debug(state: web::Data<WebState>) -> impl Responder {
    match state
        .nats
        .request::<_, RelayPacket, _>(RELAY_SUBJECT, &RelayPacket::GetDebug)
        .await
    {
        Ok(RelayPacket::ServeDebug { debug }) => HttpResponse::Ok()
            .content_type("application/json")
            .body(debug),
        Ok(_) => HttpResponse::BadGateway().body("relay returned the wrong packet type"),
        Err(error) => {
            log::warn!("relay debug query failed: {error:#}");
            HttpResponse::ServiceUnavailable().finish()
        }
    }
}
