use std::collections::HashMap;

use serde::{Serialize, de::DeserializeOwned};
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::board;

pub trait Rewardable {
    fn release_reward(&self) {}
}

pub struct RequestStub {
    pub title: String,
    pub description: String,
    pub labels: HashMap<String, String>,
    pub tags: Vec<String>,
    pub client_id: String,
}

impl RequestStub {
    pub fn new(
        title: &str,
        description: &str,
        labels: Option<HashMap<String, String>>,
        tags: Option<Vec<String>>,
        client_id: &str,
    ) -> RequestStub {
        let labels: HashMap<String, String> = labels.unwrap_or_default();
        let tags: Vec<String> = tags.unwrap_or_default();

        RequestStub {
            title: title.to_string(),
            description: description.to_string(),
            labels,
            tags,
            client_id: client_id.to_string(),
        }
    }

    pub fn to_request<T: Rewardable + Serialize + DeserializeOwned>(
        self,
        id: Uuid,
        reward: Option<T>,
    ) -> Request<T> {
        Request {
            id,
            title: self.title,
            description: self.description,
            labels: self.labels,
            tags: self.tags,
            client_id: self.client_id,
            reward,
        }
    }

    pub fn to_draft(self, id: Uuid) -> RequestDraft {
        RequestDraft {
            id,
            title: self.title,
            description: self.description,
            labels: self.labels,
            tags: self.tags,
            client_id: self.client_id,
        }
    }
}

pub struct RequestDraft {
    id: Uuid,
    pub title: String,
    pub description: String,
    pub labels: HashMap<String, String>,
    pub tags: Vec<String>,
    pub client_id: String,
}

impl RequestDraft {
    pub fn new(
        title: &str,
        description: &str,
        labels: Option<HashMap<String, String>>,
        tags: Option<Vec<String>>,
        client_id: &str,
    ) -> RequestDraft {
        let labels: HashMap<String, String> = labels.unwrap_or_default();
        let tags: Vec<String> = tags.unwrap_or_default();

        RequestDraft {
            id: Uuid::new_v4(),
            title: title.to_string(),
            description: description.to_string(),
            labels,
            tags,
            client_id: client_id.to_string(),
        }
    }

    pub async fn publish<T: Rewardable + Serialize + DeserializeOwned>(
        self,
        reward: Option<T>,
        pool: &SqlitePool,
    ) -> anyhow::Result<Request<T>> {
        let request = Request {
            id: self.id,
            title: self.title,
            description: self.description,
            labels: self.labels,
            tags: self.tags,
            client_id: self.client_id,
            reward,
        };

        board::publish(pool, &request).await?;

        Ok(request)
    }
}

pub struct Request<T: Rewardable + Serialize + DeserializeOwned> {
    id: Uuid,
    pub title: String,
    pub description: String,
    pub labels: HashMap<String, String>,
    pub tags: Vec<String>,
    pub client_id: String,
    pub reward: Option<T>,
}

impl<T: Rewardable + Serialize + DeserializeOwned> Request<T> {
    pub fn get_id(&self) -> &Uuid {
        &self.id
    }

    pub async fn claim(
        self,
        member_id: &str,
        pool: &SqlitePool,
    ) -> anyhow::Result<AcceptedRequest<T>> {
        board::delist_request(pool, &self.id).await?;
        Ok(AcceptedRequest {
            request: self,
            member_id: member_id.to_string(),
        })
    }
}

pub struct AcceptedRequest<T: Rewardable + Serialize + DeserializeOwned> {
    pub request: Request<T>,
    pub member_id: String,
}

impl<T: Rewardable + Serialize + DeserializeOwned> AcceptedRequest<T> {
    pub async fn abandon(self, pool: &SqlitePool) -> anyhow::Result<Request<T>> {
        board::publish(pool, &self.request).await?;
        Ok(self.request)
    }

    pub fn complete(self) {
        if let Some(r) = self.request.reward {
            r.release_reward();
        }
    }
}
