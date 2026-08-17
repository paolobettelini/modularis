use std::time::Duration;

use anyhow::Context;
use async_nats::{Message, Subscriber, ToSubject};
use serde::{Serialize, de::DeserializeOwned};

#[derive(Debug, Clone)]
pub struct NatsClient {
    client: async_nats::Client,
    request_timeout: Duration,
}

impl NatsClient {
    pub async fn connect(url: &str, request_timeout: Duration) -> anyhow::Result<Self> {
        let client = async_nats::connect(url)
            .await
            .with_context(|| format!("failed to connect to NATS at {url}"))?;
        Ok(Self {
            client,
            request_timeout,
        })
    }

    pub async fn subscribe<S: ToSubject>(&self, subject: S) -> anyhow::Result<Subscriber> {
        Ok(self.client.subscribe(subject).await?)
    }

    pub async fn publish<T, S>(&self, subject: S, value: &T) -> anyhow::Result<()>
    where
        T: Serialize,
        S: ToSubject,
    {
        let payload = serde_cbor::to_vec(value).context("failed to encode CBOR packet")?;
        self.client.publish(subject, payload.into()).await?;
        Ok(())
    }

    pub async fn request<Req, Resp, S>(&self, subject: S, value: &Req) -> anyhow::Result<Resp>
    where
        Req: Serialize,
        Resp: DeserializeOwned,
        S: ToSubject,
    {
        let payload = serde_cbor::to_vec(value).context("failed to encode CBOR request")?;
        let message = tokio::time::timeout(
            self.request_timeout,
            self.client.request(subject, payload.into()),
        )
        .await
        .context("NATS request timed out")??;
        Self::decode(&message)
    }

    pub fn decode<T: DeserializeOwned>(message: &Message) -> anyhow::Result<T> {
        serde_cbor::from_slice(&message.payload).context("failed to decode CBOR packet")
    }
}
