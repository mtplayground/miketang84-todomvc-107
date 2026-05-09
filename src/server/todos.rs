use crate::todo::{Filter, Todo};
use leptos::prelude::*;

#[cfg(feature = "ssr")]
fn todo_pool() -> Result<sqlx::SqlitePool, ServerFnError> {
    use leptos::context::use_context;

    use_context::<sqlx::SqlitePool>()
        .ok_or_else(|| ServerFnError::new("missing SqlitePool in server context"))
}

#[cfg(any(feature = "ssr", test))]
fn normalize_title(title: &str) -> Result<String, ServerFnError> {
    let normalized = title.trim();

    if normalized.is_empty() {
        return Err(ServerFnError::new("todo title must not be empty"));
    }

    Ok(normalized.to_owned())
}

#[server]
pub async fn list_todos(
    filter: Option<Filter>,
) -> Result<Vec<Todo>, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use crate::repository::todos as repository;

        let pool = todo_pool()?;

        match filter.unwrap_or_default() {
            Filter::All => repository::list(&pool).await.map_err(ServerFnError::from),
            filter => repository::list_by_filter(&pool, filter)
                .await
                .map_err(ServerFnError::from),
        }
    }

    #[cfg(not(feature = "ssr"))]
    {
        let _ = filter;
        Err(ServerFnError::new(
            "list_todos can only be called from the server runtime",
        ))
    }
}

#[server]
pub async fn add_todo(title: String) -> Result<Todo, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use crate::repository::todos as repository;

        let title = normalize_title(&title)?;
        let pool = todo_pool()?;
        return repository::insert(&pool, &title).await.map_err(ServerFnError::from);
    }

    #[cfg(not(feature = "ssr"))]
    {
        let _ = title;
        Err(ServerFnError::new(
            "add_todo can only be called from the server runtime",
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::normalize_title;

    #[test]
    fn normalize_title_trims_whitespace() {
        let normalized = normalize_title("  buy milk  ").expect("title should be valid");

        assert_eq!(normalized, "buy milk");
    }

    #[test]
    fn normalize_title_rejects_empty_input() {
        let error = normalize_title(" \n\t ").expect_err("blank titles must fail");

        assert!(
            error.to_string().contains("must not be empty"),
            "unexpected error message: {error}",
        );
    }
}
