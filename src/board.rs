use serde::{Serialize, de::DeserializeOwned};
use sqlx::SqlitePool;
use uuid::Uuid;

use std::collections::HashMap;

use crate::request::{Request, RequestStub, Rewardable};

pub async fn publish<T: Rewardable + Serialize + DeserializeOwned>(
    pool: &SqlitePool,
    request: &Request<T>,
) -> anyhow::Result<()> {
    let reward = request
        .reward
        .as_ref()
        .map(|reward| serde_json::to_string(reward))
        .transpose()?;

    let id = request.get_id().to_string();
    let title = &request.title;
    let client_id = &request.client_id.to_string();
    let description = &request.description;

    sqlx::query!(
        "INSERT INTO Requests (id, title, description, reward, client_id) VALUES (?, ?, ?, ?, ?)",
        id,
        title,
        description,
        reward,
        client_id
    )
    .execute(pool)
    .await?;

    for (key, value) in &request.labels {
        sqlx::query!(
            "INSERT INTO Labels (request_id, key, value) VALUES (?, ?, ?)",
            id,
            key,
            value
        )
        .execute(pool)
        .await?;
    }

    for tag in &request.tags {
        sqlx::query!("INSERT INTO Tags (request_id, tag) VALUES (?, ?)", id, tag)
            .execute(pool)
            .await?;
    }

    Ok(())
}

pub async fn delist_request(pool: &SqlitePool, request_id: &Uuid) -> anyhow::Result<()> {
    let request_id = request_id.to_string();
    sqlx::query!("DELETE FROM Requests WHERE id = ?", request_id)
        .execute(pool)
        .await?;

    sqlx::query!("DELETE FROM Labels WHERE request_id = ?", request_id)
        .execute(pool)
        .await?;

    sqlx::query!("DELETE FROM Tags WHERE request_id = ?", request_id)
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn get_all_requests<T: Rewardable + Serialize + DeserializeOwned>(
    pool: &SqlitePool,
) -> anyhow::Result<Vec<Request<T>>> {
    let rows = sqlx::query!("SELECT id, title, description, reward, client_id FROM Requests")
        .fetch_all(pool)
        .await?;

    let mut requests: Vec<Request<T>> = Vec::new();

    for row in rows {
        if let Some(id) = &row.id {
            let labels: HashMap<String, String> = sqlx::query!("SELECT key, value FROM Labels WHERE request_id = ?", id)
                .fetch_all(pool)
                .await?;

            let tags: Vec<String> = sqlx::query!("SELECT tag FROM Tags WHERE request_id = ?", id)
                .fetch_all(pool)
                .await?;

            let labels: HashMap<String, String> =
                labels.into_iter().map(|r| (r.key, r.value)).collect();

            let tags: Vec<String> = tags.into_iter().map(|r| r.tag).collect();

            let client_id = Uuid::parse_str(&row.client_id)?;

            let request_stub = RequestStub::new(
                &row.title,
                &row.description,
                Some(labels),
                Some(tags),
                client_id,
            );

            let reward = row
                .reward
                .as_deref()
                .map(|r| serde_json::from_str::<T>(r))
                .transpose()?;

            let id = row.id.as_ref().unwrap();

            let request = RequestStub::to_request::<T>(request_stub, Uuid::parse_str(id)?, reward);
            requests.push(request);
        }
    }

    Ok(requests)
}
