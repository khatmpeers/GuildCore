use serde::{Serialize, Deserialize};
use sqlx::SqlitePool;
use crate::request::request::{RequestDraft, Rewardable};

mod request;
mod board;

#[derive(Serialize, Deserialize)]
struct K;
impl Rewardable for K {}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
   let pool = SqlitePool::connect("sqlite:board.db").await?;

   let request_draft = request::request::RequestDraft::new(
       "Sample Request",
       "This is a sample request description.",
       None,
       None,
       "client_123"
   );

   let request = RequestDraft::publish::<K>(request_draft, None, &pool).await?;

   Ok(())
}
