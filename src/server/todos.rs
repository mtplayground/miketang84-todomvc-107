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

#[cfg(any(feature = "ssr", test))]
fn normalize_title_or_none(title: &str) -> Option<String> {
    let normalized = title.trim();

    if normalized.is_empty() {
        None
    } else {
        Some(normalized.to_owned())
    }
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

#[server]
pub async fn toggle_todo(
    id: i64,
    completed: bool,
) -> Result<(), ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use crate::repository::todos as repository;

        let pool = todo_pool()?;
        return repository::set_completed(&pool, id, completed)
            .await
            .map_err(ServerFnError::from);
    }

    #[cfg(not(feature = "ssr"))]
    {
        let _ = (id, completed);
        Err(ServerFnError::new(
            "toggle_todo can only be called from the server runtime",
        ))
    }
}

#[server]
pub async fn edit_todo(id: i64, title: String) -> Result<(), ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use crate::repository::todos as repository;

        let pool = todo_pool()?;

        return match normalize_title_or_none(&title) {
            Some(title) => repository::update_title(&pool, id, &title)
                .await
                .map_err(ServerFnError::from),
            None => repository::delete(&pool, id).await.map_err(ServerFnError::from),
        };
    }

    #[cfg(not(feature = "ssr"))]
    {
        let _ = (id, title);
        Err(ServerFnError::new(
            "edit_todo can only be called from the server runtime",
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::{normalize_title, normalize_title_or_none};

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

    #[test]
    fn normalize_title_or_none_trims_non_empty_values() {
        let normalized = normalize_title_or_none("  finish report  ");

        assert_eq!(normalized, Some(String::from("finish report")));
    }

    #[test]
    fn normalize_title_or_none_returns_none_for_blank_input() {
        let normalized = normalize_title_or_none("   ");

        assert_eq!(normalized, None);
    }
}
