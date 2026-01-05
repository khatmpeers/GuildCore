use guild_core::request::{RequestDraft, Rewardable};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
struct K;
impl Rewardable for K {}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let pool = SqlitePool::connect("sqlite:board.db").await?;

    let request_draft = RequestDraft::new(
        "Sample Request",
        "This is a sample request description.",
        None,
        None,
        Uuid::new_v4(),
    );

    let request = request_draft.publish::<K>(None, &pool).await?;

    Ok(())
}
