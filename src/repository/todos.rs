use crate::todo::{Filter, Todo};
use sqlx::PgPool;

pub async fn list(pool: &PgPool) -> Result<Vec<Todo>, sqlx::Error> {
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
    pool: &PgPool,
    filter: Filter,
) -> Result<Vec<Todo>, sqlx::Error> {
    match filter {
        Filter::All => list(pool).await,
        Filter::Active => {
            sqlx::query_as::<_, Todo>(
                r#"
                SELECT id, title, completed
                FROM todos
                WHERE completed = FALSE
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
                WHERE completed = TRUE
                ORDER BY id ASC
                "#,
            )
            .fetch_all(pool)
            .await
        }
    }
}

pub async fn insert(pool: &PgPool, title: &str) -> Result<Todo, sqlx::Error> {
    sqlx::query_as::<_, Todo>(
        r#"
        INSERT INTO todos (title, completed, created_at)
        VALUES ($1, FALSE, NOW())
        RETURNING id, title, completed
        "#,
    )
    .bind(title)
    .fetch_one(pool)
    .await
}

pub async fn update_title(
    pool: &PgPool,
    id: i64,
    title: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        UPDATE todos
        SET title = $1
        WHERE id = $2
        "#,
    )
    .bind(title)
    .bind(id)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn set_completed(
    pool: &PgPool,
    id: i64,
    completed: bool,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        UPDATE todos
        SET completed = $1
        WHERE id = $2
        "#,
    )
    .bind(completed)
    .bind(id)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn set_all_completed(pool: &PgPool, completed: bool) -> Result<u64, sqlx::Error> {
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

pub async fn delete(pool: &PgPool, id: i64) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        DELETE FROM todos
        WHERE id = $1
        "#,
    )
    .bind(id)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn delete_completed(pool: &PgPool) -> Result<u64, sqlx::Error> {
    let result = sqlx::query(
        r#"
        DELETE FROM todos
        WHERE completed = TRUE
        "#,
    )
    .execute(pool)
    .await?;

    Ok(result.rows_affected())
}

#[cfg(all(test, feature = "ssr"))]
mod tests {
    use super::*;
    use sqlx::{postgres::PgPoolOptions, PgPool};
    use std::env;

    async fn test_pool() -> Result<PgPool, Box<dyn std::error::Error>> {
        let database_url = env::var("TEST_DATABASE_URL")?;
        let pool = PgPoolOptions::new()
            .max_connections(1)
            .connect(&database_url)
            .await?;

        sqlx::migrate!().run(&pool).await?;
        sqlx::query("TRUNCATE TABLE todos RESTART IDENTITY")
            .execute(&pool)
            .await?;

        Ok(pool)
    }

    #[tokio::test]
    #[ignore = "requires TEST_DATABASE_URL pointing at a Postgres test database"]
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
    #[ignore = "requires TEST_DATABASE_URL pointing at a Postgres test database"]
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
    #[ignore = "requires TEST_DATABASE_URL pointing at a Postgres test database"]
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
