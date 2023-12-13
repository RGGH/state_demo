use anyhow::{Result, Error};
use std::sync::Arc;

use axum::{extract::State, routing::{get,post},  Router};
use axum::response::IntoResponse;


#[derive(Clone)]
struct AppState {
    msg: &'static str,
    counter: usize,
}

async fn hello_world(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    format!("{} (Counter: {})", state.msg, state.counter)
}

async fn increment_counter(State(mut state): State<Arc<AppState>>) -> impl IntoResponse {
    // Increment the counter in the shared state
    Arc::make_mut(&mut state).counter += 1;
    format!("Counter incremented to: {}", state.counter)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let state = Arc::new(AppState {
        msg: "Hello, world!",
        counter: 0,
    });

    let router = Router::new()
        .route("/", get(hello_world))
        .route("/increment", post(increment_counter))
        .with_state(state.clone());

    axum::Server::bind(&"0.0.0.0:3000".parse()?)
        .serve(router.into_make_service())
        .await?;

    Ok(())
}

