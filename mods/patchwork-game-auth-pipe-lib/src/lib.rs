use patchwork_game_auth_http_lib::SecretString;
use std::io::Read;
use thiserror::Error;

const AUTH_PIPE_VERSION: &str = "1";
const MAX_TICKET_BYTES: usize = 4 * 1024;

#[derive(Debug)]
pub enum AuthPipeBootstrap {
    Anonymous,
    Authenticated {
        backend_address: String,
        launch_ticket: SecretString,
    },
}

#[derive(Debug, Error)]
pub enum AuthPipeError {
    #[error("PATCHWORK_AUTH_FD is not a valid file descriptor")]
    InvalidDescriptor,
    #[error("unsupported PATCHWORK_AUTH_PIPE_VERSION")]
    UnsupportedVersion,
    #[error("BACKEND_ADDR is required for an authenticated launch")]
    MissingBackendAddress,
    #[error("authenticated game launch is not supported on this platform")]
    UnsupportedPlatform,
    #[error("could not read the Patchwork authentication pipe: {0}")]
    Read(#[from] std::io::Error),
    #[error("Patchwork launch ticket has an invalid length")]
    InvalidLength,
    #[error("Patchwork launch ticket is not valid UTF-8")]
    InvalidUtf8,
}

/// Reads the one-use ticket exactly once and closes the inherited descriptor.
/// The ticket type deliberately has a redacted debug implementation.
pub fn read_auth_pipe_from_environment() -> Result<AuthPipeBootstrap, AuthPipeError> {
    let Some(descriptor) = std::env::var_os("PATCHWORK_AUTH_FD") else {
        return Ok(AuthPipeBootstrap::Anonymous);
    };
    if std::env::var("PATCHWORK_AUTH_PIPE_VERSION").as_deref() != Ok(AUTH_PIPE_VERSION) {
        return Err(AuthPipeError::UnsupportedVersion);
    }
    let backend_address = std::env::var("BACKEND_ADDR")
        .ok()
        .filter(|value| !value.trim().is_empty())
        .ok_or(AuthPipeError::MissingBackendAddress)?;
    let descriptor = descriptor
        .to_str()
        .and_then(|value| value.parse::<i32>().ok())
        .filter(|descriptor| *descriptor >= 3)
        .ok_or(AuthPipeError::InvalidDescriptor)?;

    #[cfg(unix)]
    {
        use std::os::fd::FromRawFd;

        // Ownership is intentionally taken so dropping `pipe` closes the
        // inherited descriptor after this one read.
        let mut pipe = unsafe { std::fs::File::from_raw_fd(descriptor) };
        let mut length = [0_u8; 4];
        pipe.read_exact(&mut length)?;
        let length = u32::from_be_bytes(length) as usize;
        if length == 0 || length > MAX_TICKET_BYTES {
            return Err(AuthPipeError::InvalidLength);
        }
        let mut ticket = vec![0_u8; length];
        pipe.read_exact(&mut ticket)?;
        let ticket = String::from_utf8(ticket).map_err(|_| AuthPipeError::InvalidUtf8)?;
        Ok(AuthPipeBootstrap::Authenticated {
            backend_address,
            launch_ticket: SecretString::new(ticket),
        })
    }

    #[cfg(not(unix))]
    {
        let _ = descriptor;
        let _ = backend_address;
        Err(AuthPipeError::UnsupportedPlatform)
    }
}
