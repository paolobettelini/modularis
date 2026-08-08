# Patchwork account authentication

The demo can authenticate a game connection with the same Patchwork account
used by the desktop launcher. A player does not create a separate account for
every server, and the game client does not send a player nickname in
`JoinRequest`. On an authenticated server, the backend-redeemed Patchwork
account is the authority for both the display nickname and stable account UUID.

Authentication is optional at composition time. The shared protocol contains
the packet contracts, while separate client and server modpacks install the
actual policy. The vanilla server profile and TheCrown require authentication;
a custom server composition that does not import the server auth modpack
remains a normal unauthenticated server.

## Composition

The feature is split into three modpacks:

| Modpack | Responsibility |
| --- | --- |
| `patchwork-auth-network` | Shared packets, transport lifecycle messages, frame-security state, and ECS account contracts |
| `client-patchwork-auth` | Launcher-pipe bootstrap, process session, game-server handshake, and client join gate |
| `server-patchwork-auth` | Server lease, handshake registration/redeem, and authenticated admission rule |

`network.toml` imports the shared protocol modpack. `client.toml` imports the
client behavior. `server.toml` and `thecrown.toml` import the server behavior.
Keeping these choices separate means another server can reuse the protocol but
provide a different admission policy, or omit account authentication
completely.

The implementation also keeps ordinary Rust libraries separate from runtime
mods:

- `patchwork-game-auth-crypto-lib` implements the protocol's pure
  cryptographic operations;
- `patchwork-game-auth-http-lib` contains blocking backend calls and redacted
  secret wrappers;
- `patchwork-game-auth-pipe-lib` consumes the inherited anonymous pipe;
- `patchwork-game-auth-api` defines shared resources and ECS messages;
- `network-frame-security-api` defines the transport-facing plaintext,
  paused, secure, and failed states.

The blocking HTTP calls run on named worker threads. Bevy systems poll channels
for results, so a slow backend does not block the game update schedule.

## Required environment

An authenticated launcher process starts the client with:

```text
BACKEND_ADDR=https://backend.example
PATCHWORK_AUTH_FD=3
PATCHWORK_AUTH_PIPE_VERSION=1
```

The inherited file descriptor contains exactly:

```text
u32 ticket length, big-endian
ticket-length bytes of UTF-8 launch ticket
```

The client reads the descriptor once, takes ownership of it, and closes it. A
ticket larger than 4 KiB, an invalid descriptor, an unsupported pipe version,
or invalid UTF-8 fails the authenticated bootstrap.

If `PATCHWORK_AUTH_FD` is absent, the client is in anonymous mode. It can use a
server composition that does not require Patchwork authentication. If the
descriptor is present, the current client deliberately waits for the server
authentication challenge before sending `JoinRequest`; it does not silently
downgrade an authenticated launch to an unauthenticated join.

The authenticated server needs:

```text
BACKEND_ADDR=https://backend.example
```

The backend address must use `http://` or `https://`. Production deployments
should use HTTPS. HTTP is useful only for trusted local development.

## Client process bootstrap

Before a game-server identity is trusted, the client exchanges its one-use
launcher ticket for a process session:

```text
anonymous pipe
    -> POST /game/process-sessions
    -> process_token + process_session_id + account UUID + nickname
```

The `process_token` remains private to the client process and is never sent to
the game server. The token and launch ticket use secret wrapper types whose
`Debug` output is always `[REDACTED]`.

`ClientProcessAuthState` exposes the lifecycle without exposing the token:

```text
Anonymous | Starting | Ready | Failed
```

When the backend accepts the process session, the client publishes:

```rust
ClientPatchworkProcessAuthenticated {
    account_uuid,
    nickname,
    process_session_id,
}
```

Other client features may observe this event, but they must not treat it as
proof that a particular game server accepted the account.

## Server instance lifecycle

`server-patchwork-auth-instance-mod` owns one ephemeral backend lease:

```text
startup       -> POST /server/instances
every 2 min   -> POST /server/instances/{server_id}/heartbeat
clean stop    -> DELETE /server/instances/{server_id}
```

The returned `server_secret` exists only in RAM and is never logged or sent to
a client. If a heartbeat fails, the server stops using those credentials and
registers a new instance. New connections wait briefly for valid credentials;
they are rejected if the service remains unavailable.

The lease worker is infrastructure, not admission policy. The separate
admission mod decides that every normal player join must already have a
redeemed Patchwork account.

## Direct join flow

Each TCP connection gets fresh X25519 keys, fresh nonces, and a fresh
handshake ID:

```text
Client                         Game server                    Backend
  |                                 |                            |
  | TCP connect                     |                            |
  |-------------------------------->|                            |
  |                                 | register handshake         |
  |                                 |--------------------------->|
  | KeyExchangeRequest              |                            |
  |<--------------------------------|                            |
  | authorize transcript with process_token                     |
  |------------------------------------------------------------->|
  | KeyExchangeResponse             |                            |
  |-------------------------------->| redeem transcript           |
  |                                 |--------------------------->|
  |              AES-256-GCM starts in both directions           |
  | encrypted ClientFinish          |                            |
  |-------------------------------->|                            |
  | encrypted LoginSuccess          |                            |
  |<--------------------------------|                            |
  | JoinRequest (no player name)                                 |
  |-------------------------------->| authenticated admission    |
```

The backend is the authority that binds the process account to the concrete
server instance and concrete key exchange. The server trusts UUID and nickname
only after `/redeem` returns `accepted: true`.

The canonical transcript includes:

- protocol version;
- raw handshake UUID;
- server ID;
- both X25519 public keys;
- both 32-byte nonces.

Both sides hash the exact binary transcript with SHA-256. A mismatched hash,
expired handshake, all-zero X25519 result, backend rejection, or unexpected
packet order closes the connection.

## Encrypted frame layer

After authorization, both directions use keys and IVs derived with
HKDF-SHA256. Client-to-server and server-to-client have different key material
and independent sequence counters.

The transport pipeline becomes:

```text
typed packet
  -> CBOR
  -> AES-256-GCM using direction + sequence AAD
  -> 4-byte length-prefixed TCP frame
```

The initial challenge and response are plaintext framed packets. The transport
enters a temporary `Paused` state while the server waits for backend redeem, so
it cannot accidentally decode an encrypted packet as plaintext. The first
encrypted packets are `ClientFinish` and `LoginSuccess`; all later gameplay
packets use the same per-connection channel.

An inbound sequence number advances only after both GCM authentication and
CBOR packet decoding succeed. Invalid tags, malformed secure packets, or
counter exhaustion fail closed. The key, IV, and sequence state is never reused
for another connection.

`network-frame-security-api` contains this neutral frame transform. The TCP
mods call it without knowing what a Patchwork account means. A different
transport can preserve the auth protocol by providing the same frame-security
integration and transport lifecycle events.

## Trusted identity and admission

The server stores authenticated records in `ServerAuthenticatedAccounts`:

```text
socket address -> AuthenticatedAccount
PlayerId       -> AuthenticatedAccount
```

The address mapping exists after `ClientFinish`. The player mapping is created
only after the ordinary session system admits the join. Both mappings store the
complete `AuthenticatedAccount`, so code keyed by `PlayerId` can retrieve the
backend nickname and the stable `account_uuid` without consulting client data.

`server-patchwork-auth-admission-mod` registers a composable
`ServerPlayerAdmissionRule`. Its `prepare` phase requires a redeemed account for
the source address and writes the backend nickname into the mutable
`ServerJoinCandidate`. The admission framework completes every rule's prepare
phase before any validation rule runs, so duplicate-name and other policies see
the authenticated identity rather than a client value or anonymous fallback.

`JoinRequest` itself is a unit packet and contains no nickname. The client join
gate retains the complete `AuthenticatedAccount` received in `LoginSuccess` and
only releases the pending join after authentication succeeds; it does not copy
the nickname into the join packet. Persistent server data must use
`account_uuid`, never the nickname, because display names can change.

Two server events expose the two useful boundaries:

```rust
ServerPatchworkAccountAuthenticated { address, account }
ServerPatchworkPlayerJoined { player_id, account }
```

Use the first for pre-session policy or auditing. Use the second for gameplay
state that needs a stable `PlayerId`, such as loading persistent progress. The
normal `ServerPlayerJoined` lifecycle event still exists for features that do
not care about account identity.

The client publishes `ClientPatchworkGameAuthenticated` after validating
`LoginSuccess` against its process account.

## Logging and failure handling

The auth mods use Bevy's normal `info!` and `warn!` logging pipeline. Logs
include state transitions, public server IDs, account UUIDs, handshake IDs, and
network addresses where useful. They never include:

- launch tickets;
- process tokens;
- server secrets;
- private X25519 keys;
- AES keys or IVs.

Client failures set the normal disconnect reason and open the existing
disconnect screen. Server failures queue `AuthenticationFailed` when a safe
channel exists, flush it, and then close the socket. Secrets are not included
in user-facing backend errors.

## Extending the system

A custom server can depend on the auth API and events without selecting the
blanket admission mod. For example, it can authenticate every connection but
route accounts into different runtime scopes based on subscription, ban state,
party, or persisted profile. The cryptographic handshake should remain a
neutral identity mechanism; lobby and gameplay policy belong in separate mods
or in a custom server orchestrator.

Transfer tickets are not wired into the game yet. A future transfer feature
should add separate packet and routing mods, keep the old server authoritative
until handoff succeeds, and perform a completely fresh handshake with the
destination. It must not reuse X25519 keys, AES material, IVs, or sequence
counters from the previous connection.
