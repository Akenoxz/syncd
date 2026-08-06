use sqlx::SqlitePool;

pub async fn init_table(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS items (
            id TEXT PRIMARY KEY,
            title TEXT NOT NULL,
            done INTEGER NOT NULL,
            created_at INTEGER NOT NULL
        )",
    )
    .execute(pool)
    .await?;

    Ok(())
}
//get fn
pub async fn get_items(pool: &SqlitePool) -> Result<Vec<(String, String, bool)>, sqlx::Error> {
    let rows = sqlx::query_as::<_, (String, String, bool)>(
        "SELECT id, title, done FROM items ORDER BY created_at DESC",
    )
    .fetch_all(pool)
    .await?;

    Ok(rows)
}

//insert
pub async fn insert_item(
    pool: &SqlitePool,
    id: &str,
    title: &str,
    done: bool,
    created_at: i64,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO items (id, title, done, created_at)
         VALUES ($1, $2, $3, $4)",
    )
    .bind(id)
    .bind(title)
    .bind(done)
    .bind(created_at)
    .execute(pool)
    .await?;

    Ok(())
}
