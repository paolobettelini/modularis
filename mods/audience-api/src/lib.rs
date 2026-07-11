use serde::{Deserialize, Serialize};

pub type AudienceMemberId = u64;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AudienceId(pub String);

impl AudienceId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Audience {
    Personal(AudienceMemberId),
    Shared(AudienceId),
}

impl Audience {
    pub fn personal(member: AudienceMemberId) -> Self {
        Self::Personal(member)
    }

    pub fn shared(id: impl Into<String>) -> Self {
        Self::Shared(AudienceId::new(id))
    }

    pub fn contains_personal_member(&self, member: AudienceMemberId) -> bool {
        matches!(self, Self::Personal(owner) if *owner == member)
    }
}

pub trait AudienceApi: Send + Sync + 'static {}
