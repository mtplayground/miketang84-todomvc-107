#[cfg(feature = "ssr")]
use axum::extract::FromRef;
#[cfg(feature = "ssr")]
use leptos::{config::LeptosOptions, context::provide_context};
#[cfg(feature = "ssr")]
use sqlx::{
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
    SqlitePool,
};
#[cfg(feature = "ssr")]
use std::{env, io, str::FromStr};

#[cfg(feature = "ssr")]
#[derive(Clone)]
struct AppState {
    leptos_options: LeptosOptions,
    pool: SqlitePool,
}

#[cfg(feature = "ssr")]
impl FromRef<AppState> for LeptosOptions {
    fn from_ref(state: &AppState) -> Self {
        state.leptos_options.clone()
    }
}

#[cfg(feature = "ssr")]
impl AppState {
    async fn new(
        leptos_options: LeptosOptions,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let pool = init_database_pool().await?;

        Ok(Self {
            leptos_options,
            pool,
        })
    }
}

#[cfg(feature = "ssr")]
async fn init_database_pool() -> Result<SqlitePool, Box<dyn std::error::Error>> {
    let database_url = env::var("DATABASE_URL").map_err(|_| {
        io::Error::new(
            io::ErrorKind::NotFound,
            "DATABASE_URL environment variable must be set",
        )
    })?;

    let connect_options = SqliteConnectOptions::from_str(&database_url)?
        .create_if_missing(true);

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(connect_options)
        .await?;

    Ok(pool)
}

#[cfg(feature = "ssr")]
fn load_env_file() -> Result<(), Box<dyn std::error::Error>> {
    match dotenvy::dotenv() {
        Ok(_) => Ok(()),
        Err(dotenvy::Error::Io(error))
            if error.kind() == io::ErrorKind::NotFound =>
        {
            Ok(())
        }
        Err(error) => Err(Box::new(error)),
    }
}

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use axum::Router;
    use leptos::config::get_configuration;
    use leptos::logging::log;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use miketang84_todomvc_107::app::{shell, App};

    load_env_file()?;
    let conf = get_configuration(None)?;
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;
    let app_state = AppState::new(leptos_options.clone()).await?;
    let routes = generate_route_list(App);
    let pool = app_state.pool.clone();

    let app = Router::new()
        .leptos_routes_with_context(&app_state, routes, move || {
            provide_context(pool.clone());
        }, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .fallback(leptos_axum::file_and_error_handler::<AppState, _>(shell))
        .with_state(app_state);

    log!("initialized SQLite connection pool");
    log!("listening on http://{}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app.into_make_service()).await?;
    Ok(())
}

#[cfg(not(feature = "ssr"))]
fn main() {}
