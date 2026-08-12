use bevy::prelude::*;
use loading_task_api::LoadingTask;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ClientLoadingAuthority {
    Local(String),
    Server(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ClientLoadingTaskKey {
    pub authority: ClientLoadingAuthority,
    pub id: String,
}

#[derive(Resource, Debug, Default)]
pub struct ClientLoadingTasks(HashMap<ClientLoadingTaskKey, LoadingTask>);

impl ClientLoadingTasks {
    pub fn tasks(&self) -> impl Iterator<Item = (&ClientLoadingTaskKey, &LoadingTask)> {
        self.0.iter()
    }

    pub fn is_empty(&self) -> bool { self.0.is_empty() }

    pub fn has_blocking_tasks(&self) -> bool {
        self.0.values().any(|task| task.blocking)
    }

    pub fn set(&mut self, key: ClientLoadingTaskKey, task: LoadingTask) -> bool {
        let task = task.normalized();
        if self.0.get(&key) == Some(&task) { return false; }
        self.0.insert(key, task);
        true
    }

    pub fn remove(&mut self, key: &ClientLoadingTaskKey) -> bool {
        self.0.remove(key).is_some()
    }

    pub fn keys_for_authority(&self, authority: &ClientLoadingAuthority) -> Vec<ClientLoadingTaskKey> {
        self.0.keys().filter(|key| &key.authority == authority).cloned().collect()
    }

    pub fn clear(&mut self) -> Vec<ClientLoadingTaskKey> {
        self.0.drain().map(|(key, _)| key).collect()
    }
}

#[derive(Message, Debug, Clone, PartialEq)]
pub struct SetClientLoadingTask {
    pub authority: ClientLoadingAuthority,
    pub task: LoadingTask,
}

#[derive(Message, Debug, Clone, PartialEq, Eq)]
pub struct RemoveClientLoadingTask {
    pub key: ClientLoadingTaskKey,
}

#[derive(Message, Debug, Clone, PartialEq, Eq)]
pub struct ClearClientLoadingAuthority {
    pub authority: ClientLoadingAuthority,
}

#[derive(Message, Debug, Clone, PartialEq)]
pub struct ClientLoadingTaskChanged {
    pub key: ClientLoadingTaskKey,
    pub task: Option<LoadingTask>,
}

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ClientLoadingSet { Receive, Apply, React, Render }

pub trait ClientLoadingApi: Send + Sync + 'static {}
