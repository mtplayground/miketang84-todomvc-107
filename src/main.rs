#![recursion_limit = "256"]

#[cfg(feature = "ssr")]
use axum::extract::FromRef;
#[cfg(feature = "ssr")]
use axum::http::StatusCode;
#[cfg(feature = "ssr")]
use leptos::{config::LeptosOptions, context::provide_context};
#[cfg(feature = "ssr")]
use sqlx::{postgres::PgPoolOptions, PgPool};
#[cfg(feature = "ssr")]
use std::{env, io};

#[cfg(feature = "ssr")]
#[derive(Clone)]
struct AppState {
    leptos_options: LeptosOptions,
    pool: PgPool,
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
async fn init_database_pool() -> Result<PgPool, Box<dyn std::error::Error>> {
    let database_url = env::var("DATABASE_URL").map_err(|_| {
        io::Error::new(
            io::ErrorKind::NotFound,
            "DATABASE_URL environment variable must be set",
        )
    })?;

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    sqlx::migrate!().run(&pool).await?;

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
fn init_tracing() -> Result<(), Box<dyn std::error::Error>> {
    let env_filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .or_else(|_| tracing_subscriber::EnvFilter::try_new("info"))
        .map_err(|error| {
            io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("invalid tracing filter configuration: {error}"),
            )
        })?;

    tracing_subscriber::fmt()
        .with_env_filter(env_filter)
        .with_target(false)
        .try_init()
        .map_err(|error| {
            io::Error::new(
                io::ErrorKind::AlreadyExists,
                format!("failed to initialize tracing subscriber: {error}"),
            )
        })?;

    Ok(())
}

#[cfg(feature = "ssr")]
async fn healthz() -> StatusCode {
    StatusCode::OK
}

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use axum::{routing::get, Router};
    use leptos::config::get_configuration;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use miketang84_todomvc_107::app::{shell, App};

    load_env_file()?;
    init_tracing()?;
    let conf = get_configuration(None)?;
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;
    let app_state = AppState::new(leptos_options.clone()).await?;
    let environment = leptos_options.env.clone();
    let routes = generate_route_list(App);
    let pool = app_state.pool.clone();

    let app = Router::new()
        .route("/healthz", get(healthz))
        .leptos_routes_with_context(&app_state, routes, move || {
            provide_context(pool.clone());
        }, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .fallback(leptos_axum::file_and_error_handler::<AppState, _>(shell))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    let bound_addr = listener.local_addr()?;

    tracing::info!(
        site_addr = %bound_addr,
        environment = ?environment,
        "server startup complete",
    );

    axum::serve(listener, app.into_make_service()).await?;
    Ok(())
}

#[cfg(not(feature = "ssr"))]
fn main() {}
