use anyhow::{Result,Error};
use std::sync::Arc;

use axum::{extract::State, response::IntoResponse, routing::get, Router};

async fn hello_world(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    state.msg
}

#[derive(Clone)]
struct AppState {
    msg: &'static str,
}
#[tokio::main]
async fn main()->Result<(),Error> {
    let state = Arc::new(AppState {
        msg: "Hello, world!",
    });

    let router = Router::new().route("/", get(hello_world)).with_state(state);

    axum::Server::bind(&"0.0.0.0:3000".parse()?)
        .serve(router.into_make_service())
        .await?;

    Ok(())

}
