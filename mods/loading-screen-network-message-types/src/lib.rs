use loading_task_api::LoadingTask;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetLoadingTask {
    pub authority: String,
    pub task: LoadingTask,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RemoveLoadingTask {
    pub authority: String,
    pub id: String,
}
