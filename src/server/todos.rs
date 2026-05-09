use crate::todo::{Filter, Todo};
use leptos::prelude::*;

#[server]
pub async fn list_todos(
    filter: Option<Filter>,
) -> Result<Vec<Todo>, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use crate::repository::todos as repository;
        use leptos::context::use_context;
        use sqlx::SqlitePool;

        let pool = use_context::<SqlitePool>()
            .ok_or_else(|| ServerFnError::new("missing SqlitePool in server context"))?;

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
