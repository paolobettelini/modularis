use bevy::{log::LogPlugin, prelude::*};
use bevy_mod::BevyMod;
use tokio::task::JoinHandle;

/// Installs Bevy's tracing subscriber for headless server compositions.
///
/// `MinimalPlugins` intentionally does not contain `LogPlugin`, so logging is
/// kept as an explicit and replaceable Patchwork concern.
pub struct ServerBevyLogMod;

impl ServerBevyLogMod {
    pub fn init(bevy: &mut BevyMod) -> Self {
        // This line deliberately does not use tracing: it remains visible even
        // if a stale composition omitted the logger or logger setup fails.
        eprintln!("[server] initializing Bevy logging");

        if !bevy.app.is_plugin_added::<LogPlugin>() {
            bevy.app.add_plugins(LogPlugin::default());
        }

        info!(
            "server logging initialized (set RUST_LOG to change the filter)"
        );
        Self
    }

    pub fn run(&self) -> Option<Vec<JoinHandle<()>>> {
        None
    }
}
