use anyhow::{Error, Result};
use axum::response::IntoResponse;
use std::sync::{Arc, Mutex};

use axum::{extract::State, routing::{get,post}, Router};

#[derive(Clone)]
struct AppState {
    msg: &'static str,
    counter: usize,
}

async fn hello_world(State(state): State<Arc<Mutex<AppState>>>) -> impl IntoResponse {
    let state = state.lock().unwrap();
    format!("{} (Counter: {})", state.msg, state.counter)
}

async fn increment_counter(State(state): State<Arc<Mutex<AppState>>>) -> impl IntoResponse {
    let mut state = state.lock().unwrap();
    state.counter += 1;
    format!("Counter incremented to: {}", state.counter)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let state = Arc::new(Mutex::new(AppState {
        msg: "Hello, world!",
        counter: 0,
    }));

    let router = Router::new()
        .route("/", get(hello_world))
        .route("/increment", post(increment_counter))
        .with_state(state.clone());

    axum::Server::bind(&"0.0.0.0:3000".parse()?)
        .serve(router.into_make_service())
        .await?;

    Ok(())
}
