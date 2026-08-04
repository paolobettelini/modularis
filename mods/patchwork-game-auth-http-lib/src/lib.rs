use serde::{Deserialize, Serialize, de::DeserializeOwned};
use thiserror::Error;

#[derive(Clone)]
pub struct SecretString(String);

impl SecretString {
    pub fn new(value: String) -> Self {
        Self(value)
    }

    fn expose(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Debug for SecretString {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("[REDACTED]")
    }
}

#[derive(Debug, Error, Clone)]
pub enum AuthHttpError {
    #[error("invalid Patchwork backend address")]
    InvalidBackendAddress,
    #[error("Patchwork backend request failed: {0}")]
    Transport(String),
    #[error("Patchwork backend rejected the request ({status}): {message}")]
    Rejected { status: u16, message: String },
    #[error("Patchwork backend returned an invalid response: {0}")]
    InvalidResponse(String),
}

#[derive(Clone, Debug)]
pub struct PatchworkAuthBackend {
    base_url: String,
}

impl PatchworkAuthBackend {
    pub fn new(base_url: impl Into<String>) -> Result<Self, AuthHttpError> {
        let base_url = base_url.into().trim().trim_end_matches('/').to_owned();
        if !(base_url.starts_with("http://") || base_url.starts_with("https://")) {
            return Err(AuthHttpError::InvalidBackendAddress);
        }
        Ok(Self { base_url })
    }

    pub fn process_session(
        &self,
        launch_ticket: SecretString,
    ) -> Result<ProcessSession, AuthHttpError> {
        let response: ProcessSessionResponse = self.post_json(
            "/game/process-sessions",
            None,
            &ProcessSessionRequest {
                launch_ticket: launch_ticket.expose(),
            },
        )?;
        Ok(ProcessSession {
            process_token: SecretString::new(response.process_token),
            process_session_id: response.process_session_id,
            expires_in: response.expires_in,
            account: PatchworkAccount {
                uuid: response.uuid,
                nickname: response.nickname,
            },
        })
    }

    pub fn authorize_handshake(
        &self,
        process: &ProcessSession,
        request: &AuthorizeHandshakeRequest,
    ) -> Result<(), AuthHttpError> {
        let response: AuthorizedResponse = self.post_json(
            "/game/handshakes/authorize",
            Some(process.process_token.expose()),
            request,
        )?;
        if !response.authorized {
            return Err(AuthHttpError::Rejected {
                status: 403,
                message: "handshake was not authorized".to_owned(),
            });
        }
        Ok(())
    }

    pub fn create_server_instance(&self) -> Result<ServerInstanceCredentials, AuthHttpError> {
        let response: ServerInstanceResponse = self.post_empty("/server/instances", None)?;
        Ok(ServerInstanceCredentials {
            server_id: response.server_id,
            server_secret: SecretString::new(response.server_secret),
            expires_in: response.expires_in,
        })
    }

    pub fn heartbeat_server_instance(
        &self,
        credentials: &ServerInstanceCredentials,
    ) -> Result<i64, AuthHttpError> {
        let response: HeartbeatResponse = self.post_empty(
            &format!("/server/instances/{}/heartbeat", credentials.server_id),
            Some(credentials.server_secret.expose()),
        )?;
        if !response.alive || response.server_id != credentials.server_id {
            return Err(AuthHttpError::InvalidResponse(
                "heartbeat did not confirm the current server instance".to_owned(),
            ));
        }
        Ok(response.expires_in)
    }

    pub fn close_server_instance(
        &self,
        credentials: &ServerInstanceCredentials,
    ) -> Result<(), AuthHttpError> {
        let request = self
            .request(
                "DELETE",
                &format!("/server/instances/{}", credentials.server_id),
            )?
            .set(
                "Authorization",
                &format!("Bearer {}", credentials.server_secret.expose()),
            );
        request.call().map_err(map_ureq_error)?;
        Ok(())
    }

    pub fn register_handshake(
        &self,
        credentials: &ServerInstanceCredentials,
        request: &RegisterHandshakeRequest,
    ) -> Result<i64, AuthHttpError> {
        let response: RegisterHandshakeResponse = self.post_json(
            "/server/handshakes",
            Some(credentials.server_secret.expose()),
            request,
        )?;
        if !response.registered || response.server_id != credentials.server_id {
            return Err(AuthHttpError::InvalidResponse(
                "backend registered the handshake for another server instance".to_owned(),
            ));
        }
        Ok(response.expires_in)
    }

    pub fn redeem_handshake(
        &self,
        credentials: &ServerInstanceCredentials,
        handshake_id: &str,
        request: &RedeemHandshakeRequest,
    ) -> Result<RedeemedPlayerSession, AuthHttpError> {
        let response: RedeemHandshakeResponse = self.post_json(
            &format!("/server/handshakes/{handshake_id}/redeem"),
            Some(credentials.server_secret.expose()),
            request,
        )?;
        if !response.accepted {
            return Err(AuthHttpError::Rejected {
                status: 403,
                message: "handshake redemption was not accepted".to_owned(),
            });
        }
        Ok(RedeemedPlayerSession {
            admission: response.admission,
            player_session_id: response.player_session_id,
            account: response.account,
            source_server_id: response.source_server_id,
        })
    }

    fn post_empty<T: DeserializeOwned>(
        &self,
        path: &str,
        bearer: Option<&str>,
    ) -> Result<T, AuthHttpError> {
        let mut request = self.request("POST", path)?;
        if let Some(bearer) = bearer {
            request = request.set("Authorization", &format!("Bearer {bearer}"));
        }
        decode_response(request.call().map_err(map_ureq_error)?)
    }

    fn post_json<T: DeserializeOwned>(
        &self,
        path: &str,
        bearer: Option<&str>,
        body: &impl Serialize,
    ) -> Result<T, AuthHttpError> {
        let mut request = self
            .request("POST", path)?
            .set("Content-Type", "application/json");
        if let Some(bearer) = bearer {
            request = request.set("Authorization", &format!("Bearer {bearer}"));
        }
        let body = serde_json::to_value(body)
            .map_err(|error| AuthHttpError::InvalidResponse(error.to_string()))?;
        decode_response(request.send_json(body).map_err(map_ureq_error)?)
    }

    fn request(&self, method: &str, path: &str) -> Result<ureq::Request, AuthHttpError> {
        if !path.starts_with('/') {
            return Err(AuthHttpError::InvalidBackendAddress);
        }
        Ok(ureq::request(method, &format!("{}{}", self.base_url, path))
            .timeout(std::time::Duration::from_secs(10)))
    }
}

#[derive(Clone)]
pub struct ProcessSession {
    process_token: SecretString,
    pub process_session_id: String,
    pub expires_in: i64,
    pub account: PatchworkAccount,
}

impl std::fmt::Debug for ProcessSession {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("ProcessSession")
            .field("process_token", &self.process_token)
            .field("process_session_id", &self.process_session_id)
            .field("expires_in", &self.expires_in)
            .field("account", &self.account)
            .finish()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PatchworkAccount {
    pub uuid: String,
    pub nickname: String,
}

#[derive(Clone)]
pub struct ServerInstanceCredentials {
    server_id: String,
    server_secret: SecretString,
    expires_in: i64,
}

impl ServerInstanceCredentials {
    pub fn server_id(&self) -> &str {
        &self.server_id
    }

    pub fn expires_in(&self) -> i64 {
        self.expires_in
    }
}

impl std::fmt::Debug for ServerInstanceCredentials {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("ServerInstanceCredentials")
            .field("server_id", &self.server_id)
            .field("server_secret", &self.server_secret)
            .field("expires_in", &self.expires_in)
            .finish()
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct AuthorizeHandshakeRequest {
    pub protocol_version: u16,
    pub handshake_id: String,
    pub server_id: String,
    pub server_public_key: String,
    pub client_public_key: String,
    pub server_nonce: String,
    pub client_nonce: String,
    pub handshake_hash: String,
    pub transfer_ticket: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct RegisterHandshakeRequest {
    pub handshake_id: String,
    pub protocol_version: u16,
    pub server_public_key: String,
    pub server_nonce: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct RedeemHandshakeRequest {
    pub client_public_key: String,
    pub client_nonce: String,
    pub handshake_hash: String,
}

#[derive(Debug, Clone)]
pub struct RedeemedPlayerSession {
    pub admission: String,
    pub player_session_id: String,
    pub account: PatchworkAccount,
    pub source_server_id: Option<String>,
}

#[derive(Serialize)]
struct ProcessSessionRequest<'a> {
    launch_ticket: &'a str,
}

#[derive(Deserialize)]
struct ProcessSessionResponse {
    process_token: String,
    process_session_id: String,
    expires_in: i64,
    uuid: String,
    nickname: String,
}

#[derive(Deserialize)]
struct AuthorizedResponse {
    authorized: bool,
}

#[derive(Deserialize)]
struct ServerInstanceResponse {
    server_id: String,
    server_secret: String,
    expires_in: i64,
}

#[derive(Deserialize)]
struct HeartbeatResponse {
    alive: bool,
    server_id: String,
    expires_in: i64,
}

#[derive(Deserialize)]
struct RegisterHandshakeResponse {
    registered: bool,
    server_id: String,
    expires_in: i64,
}

#[derive(Deserialize)]
struct RedeemHandshakeResponse {
    accepted: bool,
    admission: String,
    player_session_id: String,
    account: PatchworkAccount,
    source_server_id: Option<String>,
}

fn decode_response<T: DeserializeOwned>(response: ureq::Response) -> Result<T, AuthHttpError> {
    response
        .into_json::<T>()
        .map_err(|error| AuthHttpError::InvalidResponse(error.to_string()))
}

fn map_ureq_error(error: ureq::Error) -> AuthHttpError {
    match error {
        ureq::Error::Status(status, response) => {
            let message = response
                .into_json::<serde_json::Value>()
                .ok()
                .and_then(|body| {
                    body.get("message")
                        .and_then(serde_json::Value::as_str)
                        .map(str::to_owned)
                })
                .unwrap_or_else(|| "request rejected".to_owned());
            AuthHttpError::Rejected { status, message }
        }
        ureq::Error::Transport(error) => AuthHttpError::Transport(error.to_string()),
    }
}
