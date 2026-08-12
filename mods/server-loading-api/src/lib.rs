use bevy::prelude::*;
use loading_task_api::LoadingTask;
use player_network_message_types::PlayerId;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ServerLoadingTaskKey {
    pub player_id: PlayerId,
    pub authority: String,
    pub id: String,
}

#[derive(Resource, Debug, Default)]
pub struct ServerLoadingTasks(HashMap<ServerLoadingTaskKey, LoadingTask>);

impl ServerLoadingTasks {
    pub fn tasks_for_player(&self, player_id: PlayerId) -> Vec<(ServerLoadingTaskKey, LoadingTask)> {
        self.0.iter().filter(|(key, _)| key.player_id == player_id)
            .map(|(key, task)| (key.clone(), task.clone())).collect()
    }

    pub fn set(&mut self, key: ServerLoadingTaskKey, task: LoadingTask) -> bool {
        let task = task.normalized();
        if self.0.get(&key) == Some(&task) { return false; }
        self.0.insert(key, task);
        true
    }

    pub fn remove(&mut self, key: &ServerLoadingTaskKey) -> bool {
        self.0.remove(key).is_some()
    }

    pub fn remove_player(&mut self, player_id: PlayerId) {
        self.0.retain(|key, _| key.player_id != player_id);
    }
}

#[derive(Message, Debug, Clone, PartialEq)]
pub struct SetServerPlayerLoadingTask {
    pub player_id: PlayerId,
    pub authority: String,
    pub task: LoadingTask,
}

#[derive(Message, Debug, Clone, PartialEq, Eq)]
pub struct RemoveServerPlayerLoadingTask {
    pub key: ServerLoadingTaskKey,
}

#[derive(Message, Debug, Clone, PartialEq)]
pub struct ServerPlayerLoadingTaskChanged {
    pub key: ServerLoadingTaskKey,
    pub task: Option<LoadingTask>,
}

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ServerLoadingSet { Apply, Sync }

pub trait ServerLoadingApi: Send + Sync + 'static {}
