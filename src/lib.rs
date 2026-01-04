use sqlx::{Sqlite, SqlitePool, migrate::MigrateDatabase};

pub mod board;
pub mod request;

pub use board::{delist_request, get_all_requests, publish};
pub use request::{AcceptedRequest, Request, RequestDraft, Rewardable};

pub async fn init_board(database_url: &str) -> anyhow::Result<SqlitePool> {
    if !Sqlite::database_exists(database_url).await? {
        Sqlite::create_database(database_url).await?;
    }

    let pool = SqlitePool::connect(database_url).await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    Ok(pool)
}
