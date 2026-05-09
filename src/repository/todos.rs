use crate::todo::{Filter, Todo};
use sqlx::SqlitePool;

pub async fn list(pool: &SqlitePool) -> Result<Vec<Todo>, sqlx::Error> {
    sqlx::query_as::<_, Todo>(
        r#"
        SELECT id, title, completed
        FROM todos
        ORDER BY id ASC
        "#,
    )
    .fetch_all(pool)
    .await
}

pub async fn list_by_filter(
    pool: &SqlitePool,
    filter: Filter,
) -> Result<Vec<Todo>, sqlx::Error> {
    match filter {
        Filter::All => list(pool).await,
        Filter::Active => {
            sqlx::query_as::<_, Todo>(
                r#"
                SELECT id, title, completed
                FROM todos
                WHERE completed = 0
                ORDER BY id ASC
                "#,
            )
            .fetch_all(pool)
            .await
        }
        Filter::Completed => {
            sqlx::query_as::<_, Todo>(
                r#"
                SELECT id, title, completed
                FROM todos
                WHERE completed = 1
                ORDER BY id ASC
                "#,
            )
            .fetch_all(pool)
            .await
        }
    }
}

pub async fn insert(pool: &SqlitePool, title: &str) -> Result<Todo, sqlx::Error> {
    sqlx::query_as::<_, Todo>(
        r#"
        INSERT INTO todos (title, completed, created_at)
        VALUES (?1, 0, CURRENT_TIMESTAMP)
        RETURNING id, title, completed
        "#,
    )
    .bind(title)
    .fetch_one(pool)
    .await
}

pub async fn update_title(
    pool: &SqlitePool,
    id: i64,
    title: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        UPDATE todos
        SET title = ?1
        WHERE id = ?2
        "#,
    )
    .bind(title)
    .bind(id)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn set_completed(
    pool: &SqlitePool,
    id: i64,
    completed: bool,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        UPDATE todos
        SET completed = ?1
        WHERE id = ?2
        "#,
    )
    .bind(completed)
    .bind(id)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn set_all_completed(
    pool: &SqlitePool,
    completed: bool,
) -> Result<u64, sqlx::Error> {
    let result = sqlx::query(
        r#"
        UPDATE todos
        SET completed = ?1
        "#,
    )
    .bind(completed)
    .execute(pool)
    .await?;

    Ok(result.rows_affected())
}

pub async fn delete(pool: &SqlitePool, id: i64) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        DELETE FROM todos
        WHERE id = ?1
        "#,
    )
    .bind(id)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn delete_completed(pool: &SqlitePool) -> Result<u64, sqlx::Error> {
    let result = sqlx::query(
        r#"
        DELETE FROM todos
        WHERE completed = 1
        "#,
    )
    .execute(pool)
    .await?;

    Ok(result.rows_affected())
}

#[cfg(all(test, feature = "ssr"))]
mod tests {
    use super::*;
    use sqlx::{
        sqlite::{SqliteConnectOptions, SqlitePoolOptions},
        SqlitePool,
    };
    use std::str::FromStr;

    async fn test_pool() -> Result<SqlitePool, Box<dyn std::error::Error>> {
        let connect_options = SqliteConnectOptions::from_str("sqlite::memory:")?;
        let pool = SqlitePoolOptions::new()
            .max_connections(1)
            .connect_with(connect_options)
            .await?;

        sqlx::migrate!().run(&pool).await?;

        Ok(pool)
    }

    #[tokio::test]
    async fn insert_and_list_todos() -> Result<(), Box<dyn std::error::Error>> {
        let pool = test_pool().await?;

        let first = insert(&pool, "first").await?;
        let second = insert(&pool, "second").await?;
        let todos = list(&pool).await?;

        assert_eq!(todos.len(), 2);
        assert_eq!(todos[0], first);
        assert_eq!(todos[1], second);
        assert!(!todos[0].completed);

        Ok(())
    }

    #[tokio::test]
    async fn list_by_filter_respects_completion_state(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let pool = test_pool().await?;

        let active = insert(&pool, "active").await?;
        let completed = insert(&pool, "completed").await?;
        set_completed(&pool, completed.id, true).await?;

        assert_eq!(list_by_filter(&pool, Filter::All).await?.len(), 2);
        assert_eq!(list_by_filter(&pool, Filter::Active).await?, vec![active]);
        assert_eq!(
            list_by_filter(&pool, Filter::Completed).await?,
            vec![Todo {
                completed: true,
                ..completed
            }],
        );

        Ok(())
    }

    #[tokio::test]
    async fn update_and_delete_todos() -> Result<(), Box<dyn std::error::Error>> {
        let pool = test_pool().await?;

        let first = insert(&pool, "first").await?;
        let second = insert(&pool, "second").await?;

        update_title(&pool, first.id, "renamed").await?;
        set_completed(&pool, first.id, true).await?;

        let completed_count = set_all_completed(&pool, true).await?;
        assert_eq!(completed_count, 2);

        delete(&pool, second.id).await?;
        let deleted_completed = delete_completed(&pool).await?;
        assert_eq!(deleted_completed, 1);

        let remaining = list(&pool).await?;
        assert!(remaining.is_empty());

        Ok(())
    }
}
