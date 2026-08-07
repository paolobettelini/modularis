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

/// Reads the one-use launch ticket from the transport selected for the target.
///
/// Unix uses the inherited file descriptor in `PATCHWORK_AUTH_FD`. Windows
/// connects to the local named pipe in `PATCHWORK_AUTH_PIPE`. Both transports
/// use the same frame: a big-endian `u32` byte length followed by the UTF-8
/// ticket. The ticket type deliberately has a redacted debug implementation.
pub fn read_auth_pipe_from_environment() -> Result<AuthPipeBootstrap, AuthPipeError> {
    #[cfg(unix)]
    {
        return read_unix_auth_pipe_from_environment();
    }

    #[cfg(windows)]
    {
        return read_windows_auth_pipe_from_environment();
    }

    #[cfg(not(any(unix, windows)))]
    {
        if std::env::var_os("PATCHWORK_AUTH_FD").is_some()
            || std::env::var_os("PATCHWORK_AUTH_PIPE").is_some()
        {
            return Err(AuthPipeError::UnsupportedPlatform);
        }
        Ok(AuthPipeBootstrap::Anonymous)
    }
}

fn authenticated_backend_address() -> Result<String, AuthPipeError> {
    if std::env::var("PATCHWORK_AUTH_PIPE_VERSION").as_deref() != Ok(AUTH_PIPE_VERSION) {
        return Err(AuthPipeError::UnsupportedVersion);
    }
    std::env::var("BACKEND_ADDR")
        .ok()
        .filter(|value| !value.trim().is_empty())
        .ok_or(AuthPipeError::MissingBackendAddress)
}

fn read_authenticated_ticket(
    backend_address: String,
    mut pipe: impl Read,
) -> Result<AuthPipeBootstrap, AuthPipeError> {
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

#[cfg(unix)]
fn read_unix_auth_pipe_from_environment() -> Result<AuthPipeBootstrap, AuthPipeError> {
    use std::os::fd::FromRawFd;

    let Some(descriptor) = std::env::var_os("PATCHWORK_AUTH_FD") else {
        return Ok(AuthPipeBootstrap::Anonymous);
    };
    let backend_address = authenticated_backend_address()?;
    let descriptor = descriptor
        .to_str()
        .and_then(|value| value.parse::<i32>().ok())
        .filter(|descriptor| *descriptor >= 3)
        .ok_or(AuthPipeError::InvalidDescriptor)?;

    // Ownership is intentionally taken so dropping `pipe` closes the inherited
    // descriptor immediately after this one read.
    let pipe = unsafe { std::fs::File::from_raw_fd(descriptor) };
    read_authenticated_ticket(backend_address, pipe)
}

#[cfg(windows)]
fn read_windows_auth_pipe_from_environment() -> Result<AuthPipeBootstrap, AuthPipeError> {
    use std::os::windows::ffi::OsStrExt;
    use std::os::windows::io::FromRawHandle;
    use windows_sys::Win32::Foundation::{GENERIC_READ, INVALID_HANDLE_VALUE};
    use windows_sys::Win32::Storage::FileSystem::{CreateFileW, OPEN_EXISTING};

    let Some(pipe_name) = std::env::var_os("PATCHWORK_AUTH_PIPE") else {
        return Ok(AuthPipeBootstrap::Anonymous);
    };
    let backend_address = authenticated_backend_address()?;
    validate_windows_pipe_name(&pipe_name)?;

    let wide_name = pipe_name
        .as_os_str()
        .encode_wide()
        .chain(std::iter::once(0))
        .collect::<Vec<_>>();
    let handle = unsafe {
        CreateFileW(
            wide_name.as_ptr(),
            GENERIC_READ,
            0,
            std::ptr::null(),
            OPEN_EXISTING,
            0,
            std::ptr::null_mut(),
        )
    };
    if handle == INVALID_HANDLE_VALUE {
        return Err(AuthPipeError::Read(std::io::Error::last_os_error()));
    }

    // `CreateFileW` returned an owned kernel handle. `File` takes ownership so
    // it is closed even when framing or UTF-8 validation fails.
    let pipe = unsafe { std::fs::File::from_raw_handle(handle) };
    read_authenticated_ticket(backend_address, pipe)
}

#[cfg(windows)]
fn validate_windows_pipe_name(pipe_name: &std::ffi::OsStr) -> Result<(), AuthPipeError> {
    let Some(pipe_name) = pipe_name.to_str() else {
        return Err(invalid_windows_pipe_name());
    };
    if !pipe_name.starts_with(r"\\.\pipe\") || pipe_name.len() <= r"\\.\pipe\".len() {
        return Err(invalid_windows_pipe_name());
    }
    Ok(())
}

#[cfg(windows)]
fn invalid_windows_pipe_name() -> AuthPipeError {
    AuthPipeError::Read(std::io::Error::new(
        std::io::ErrorKind::InvalidInput,
        "PATCHWORK_AUTH_PIPE is not a valid local Windows named-pipe path",
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn reads_framed_ticket() {
        let ticket = b"one-use-ticket";
        let mut frame = Vec::new();
        frame.extend_from_slice(&(ticket.len() as u32).to_be_bytes());
        frame.extend_from_slice(ticket);

        let bootstrap = read_authenticated_ticket(
            "https://backend.example".to_owned(),
            Cursor::new(frame),
        )
        .expect("ticket frame should be valid");

        match bootstrap {
            AuthPipeBootstrap::Authenticated {
                backend_address, ..
            } => assert_eq!(backend_address, "https://backend.example"),
            AuthPipeBootstrap::Anonymous => panic!("expected authenticated bootstrap"),
        }
    }

    #[test]
    fn rejects_empty_ticket() {
        let error = read_authenticated_ticket(
            "https://backend.example".to_owned(),
            Cursor::new(0_u32.to_be_bytes()),
        )
        .expect_err("empty tickets must be rejected");
        assert!(matches!(error, AuthPipeError::InvalidLength));
    }

    #[test]
    fn rejects_oversized_ticket() {
        let length = (MAX_TICKET_BYTES as u32) + 1;
        let error = read_authenticated_ticket(
            "https://backend.example".to_owned(),
            Cursor::new(length.to_be_bytes()),
        )
        .expect_err("oversized tickets must be rejected");
        assert!(matches!(error, AuthPipeError::InvalidLength));
    }

    #[test]
    fn rejects_non_utf8_ticket() {
        let frame = [0, 0, 0, 1, 0xff];
        let error = read_authenticated_ticket(
            "https://backend.example".to_owned(),
            Cursor::new(frame),
        )
        .expect_err("ticket must be UTF-8");
        assert!(matches!(error, AuthPipeError::InvalidUtf8));
    }
}
