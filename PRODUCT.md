# Product Snapshot

## What this project is

A server-rendered TodoMVC application built with Leptos and Axum, hydrated on the client, and backed by SQLite through SQLx.

## What it does

The app implements the standard TodoMVC flow:

- create todos from the header input
- toggle individual todos and toggle all todos
- edit todos inline with TodoMVC-style Enter, blur, and Escape behavior
- delete individual todos and clear completed todos
- filter by `All`, `Active`, and `Completed` using URL hashes
- persist todos in SQLite so state survives reloads and restarts

## Key features

- SSR first render with client hydration
- reactive footer with remaining-count pluralization
- hash-based filter selection and filtered list rendering
- health check at `GET /healthz`
- Docker image and compose setup with a persistent volume for the SQLite database

## Architecture

- `src/app.rs`: Leptos UI, routing, and client/server interaction points
- `src/server/`: Leptos server functions for todo operations
- `src/repository/`: SQLx repository layer for todo queries and updates
- `src/todo.rs`: shared `Todo` and `Filter` types used by SSR and CSR
- `src/main.rs`: Axum server bootstrap, env loading, tracing, DB pool setup, and migrations
- `migrations/`: SQLx migration files for the SQLite schema

## Project conventions

- configuration comes from environment variables, with `.env.example` as the template
- the app listens on `0.0.0.0:8080` by default
- database access stays in the repository layer; UI mutations go through Leptos server functions
- static assets are built and served through `cargo-leptos`
