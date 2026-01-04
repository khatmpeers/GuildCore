use std::collections::HashMap;

use serde::{Serialize, de::DeserializeOwned};
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::board;

pub trait Rewardable {
    fn release_reward(&self) -> () {
        ()
    }
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
        let labels: HashMap<String, String> = if let Some(col) = labels {
            col
        } else {
            HashMap::new()
        };
        let tags: Vec<String> = if let Some(col) = tags { col } else { vec![] };

        RequestStub {
            title: title.to_string(),
            description: description.to_string(),
            labels: labels,
            tags: tags,
            client_id: client_id.to_string(),
        }
    }

    pub fn to_request<T: Rewardable + Serialize + DeserializeOwned>(
        self: Self,
        id: Uuid,
        reward: Option<T>,
    ) -> Request<T> {
        Request {
            id: id,
            title: self.title,
            description: self.description,
            labels: self.labels,
            tags: self.tags,
            client_id: self.client_id,
            reward: reward,
        }
    }

    pub fn to_draft(self: Box<Self>, id: Uuid) -> RequestDraft {
        RequestDraft {
            id: id,
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
        let labels: HashMap<String, String> = if let Some(col) = labels {
            col
        } else {
            HashMap::new()
        };
        let tags: Vec<String> = if let Some(col) = tags { col } else { vec![] };

        RequestDraft {
            id: Uuid::new_v4(),
            title: title.to_string(),
            description: description.to_string(),
            labels: labels,
            tags: tags,
            client_id: client_id.to_string(),
        }
    }

    pub async fn publish<T: Rewardable + Serialize + DeserializeOwned>(
        self: Self,
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
            reward: reward,
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
        self: Self,
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
    pub async fn abandon(self: Self, pool: &SqlitePool) -> anyhow::Result<Request<T>> {
        board::publish(pool, &self.request).await?;
        Ok(self.request)
    }

    pub fn complete(self: Self) -> () {
        self.request.reward.as_ref().map(|r| r.release_reward());
    }
}
