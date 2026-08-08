use bevy::prelude::*;
use network_frame_security_api::SecureFrameCodec;
use patchwork_game_auth_crypto_lib::SecureChannel;
use patchwork_game_auth_http_lib::ProcessSession;
use player_network_message_types::PlayerId;
use std::{
    collections::HashMap,
    io,
    net::SocketAddr,
    sync::{Arc, RwLock},
};

pub struct PatchworkSecureFrameCodec(SecureChannel);

impl PatchworkSecureFrameCodec {
    pub fn new(channel: SecureChannel) -> Self {
        Self(channel)
    }
}

impl SecureFrameCodec for PatchworkSecureFrameCodec {
    fn encode(&mut self, plaintext: &[u8]) -> io::Result<Vec<u8>> {
        self.0.encrypt(plaintext).map_err(crypto_error)
    }

    fn decode_candidate(&self, frame: &[u8]) -> io::Result<Vec<u8>> {
        self.0.decrypt_candidate(frame).map_err(crypto_error)
    }

    fn commit_inbound(&mut self) -> io::Result<()> {
        self.0.commit_inbound().map_err(crypto_error)
    }
}

fn crypto_error(error: impl std::fmt::Display) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, error.to_string())
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthenticatedAccount {
    pub account_uuid: String,
    pub nickname: String,
    pub player_session_id: String,
    pub admission: String,
    pub source_server_id: Option<String>,
}

#[derive(Clone)]
pub enum ClientProcessAuthStatus {
    Anonymous,
    Starting,
    Ready(ProcessSession),
    Failed(String),
}

impl std::fmt::Debug for ClientProcessAuthStatus {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Anonymous => formatter.write_str("Anonymous"),
            Self::Starting => formatter.write_str("Starting"),
            Self::Ready(session) => formatter.debug_tuple("Ready").field(session).finish(),
            Self::Failed(reason) => formatter.debug_tuple("Failed").field(reason).finish(),
        }
    }
}

#[derive(Resource, Clone)]
pub struct ClientProcessAuthState(Arc<RwLock<ClientProcessAuthStatus>>);

impl ClientProcessAuthState {
    pub fn new(status: ClientProcessAuthStatus) -> Self {
        Self(Arc::new(RwLock::new(status)))
    }

    pub fn status(&self) -> ClientProcessAuthStatus {
        self.0
            .read()
            .expect("client process auth state poisoned")
            .clone()
    }

    pub fn set(&self, status: ClientProcessAuthStatus) {
        *self.0.write().expect("client process auth state poisoned") = status;
    }
}

#[derive(Debug, Default)]
struct ClientJoinGateState {
    required: bool,
    ready: bool,
    account: Option<AuthenticatedAccount>,
}

#[derive(Resource, Clone, Default)]
pub struct ClientPatchworkJoinGate(Arc<RwLock<ClientJoinGateState>>);

impl ClientPatchworkJoinGate {
    pub fn require_authentication(&self) {
        let mut state = self.0.write().expect("client join gate poisoned");
        state.required = true;
        state.ready = false;
        state.account = None;
    }

    pub fn authorize(&self, account: AuthenticatedAccount) {
        let mut state = self.0.write().expect("client join gate poisoned");
        state.required = true;
        state.ready = true;
        state.account = Some(account);
    }

    pub fn reset(&self) {
        let mut state = self.0.write().expect("client join gate poisoned");
        state.ready = false;
        state.account = None;
    }

    pub fn may_join(&self) -> bool {
        let state = self.0.read().expect("client join gate poisoned");
        !state.required || state.ready
    }

    pub fn account(&self) -> Option<AuthenticatedAccount> {
        self.0
            .read()
            .expect("client join gate poisoned")
            .account
            .clone()
    }
}

#[derive(Resource, Clone, Default)]
pub struct ServerAuthenticatedAccounts {
    by_address: Arc<RwLock<HashMap<SocketAddr, AuthenticatedAccount>>>,
    by_player: Arc<RwLock<HashMap<PlayerId, AuthenticatedAccount>>>,
}

impl ServerAuthenticatedAccounts {
    pub fn authenticate(&self, address: SocketAddr, account: AuthenticatedAccount) {
        self.by_address
            .write()
            .expect("authenticated account map poisoned")
            .insert(address, account);
    }

    pub fn bind_player(
        &self,
        address: SocketAddr,
        player_id: PlayerId,
    ) -> Option<AuthenticatedAccount> {
        let account = self.account_for_address(address)?;
        self.by_player
            .write()
            .expect("authenticated player map poisoned")
            .insert(player_id, account.clone());
        Some(account)
    }

    pub fn account_for_address(&self, address: SocketAddr) -> Option<AuthenticatedAccount> {
        self.by_address
            .read()
            .expect("authenticated account map poisoned")
            .get(&address)
            .cloned()
    }

    pub fn account_for_player(&self, player_id: PlayerId) -> Option<AuthenticatedAccount> {
        self.by_player
            .read()
            .expect("authenticated player map poisoned")
            .get(&player_id)
            .cloned()
    }

    pub fn remove_address(&self, address: SocketAddr) -> Option<AuthenticatedAccount> {
        let account = self
            .by_address
            .write()
            .expect("authenticated account map poisoned")
            .remove(&address)?;
        self.by_player
            .write()
            .expect("authenticated player map poisoned")
            .retain(|_, candidate| candidate.player_session_id != account.player_session_id);
        Some(account)
    }
}

#[derive(Message, Debug, Clone, PartialEq, Eq)]
pub struct ClientPatchworkProcessAuthenticated {
    pub account_uuid: String,
    pub nickname: String,
    pub process_session_id: String,
}

#[derive(Message, Debug, Clone, PartialEq, Eq)]
pub struct ClientPatchworkGameAuthenticated {
    pub account: AuthenticatedAccount,
}

#[derive(Message, Debug, Clone, PartialEq, Eq)]
pub struct ServerPatchworkAccountAuthenticated {
    pub address: SocketAddr,
    pub account: AuthenticatedAccount,
}

#[derive(Message, Debug, Clone, PartialEq, Eq)]
pub struct ServerPatchworkPlayerJoined {
    pub player_id: PlayerId,
    pub account: AuthenticatedAccount,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn account() -> AuthenticatedAccount {
        AuthenticatedAccount {
            account_uuid: "account-uuid".to_owned(),
            nickname: "BackendName".to_owned(),
            player_session_id: "player-session".to_owned(),
            admission: "accepted".to_owned(),
            source_server_id: Some("server-id".to_owned()),
        }
    }

    #[test]
    fn client_join_gate_keeps_the_authenticated_account() {
        let gate = ClientPatchworkJoinGate::default();
        gate.require_authentication();
        assert!(!gate.may_join());

        let account = account();
        gate.authorize(account.clone());

        assert!(gate.may_join());
        assert_eq!(gate.account(), Some(account));
    }

    #[test]
    fn server_account_binding_keeps_uuid_for_player() {
        let accounts = ServerAuthenticatedAccounts::default();
        let address: SocketAddr = "127.0.0.1:9999".parse().unwrap();
        let account = account();
        accounts.authenticate(address, account.clone());

        assert_eq!(accounts.bind_player(address, 7), Some(account.clone()));
        assert_eq!(accounts.account_for_player(7), Some(account));
    }
}
