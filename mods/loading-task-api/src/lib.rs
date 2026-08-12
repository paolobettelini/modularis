use serde::{Deserialize, Serialize};

/// A single, independently owned unit of loading work.
///
/// IDs and authorities should be stable and namespaced so unrelated mods can
/// publish progress without sharing concrete types or overwriting each other.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LoadingTask {
    pub id: String,
    pub label: String,
    pub progress: f32,
    pub status: String,
    pub blocking: bool,
}

impl LoadingTask {
    pub fn new(
        id: impl Into<String>,
        label: impl Into<String>,
        progress: f32,
        status: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            progress: progress.clamp(0.0, 1.0),
            status: status.into(),
            blocking: true,
        }
    }

    pub fn non_blocking(mut self) -> Self {
        self.blocking = false;
        self
    }

    pub fn normalized(mut self) -> Self {
        self.progress = self.progress.clamp(0.0, 1.0);
        self
    }
}
