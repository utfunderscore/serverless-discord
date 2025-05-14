mod routes;
mod security;
mod model;

use axum::routing::post;
use axum::Router;
use lambda_http::{run, tracing, Error};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct ApiError {
    error: String,
}

#[derive(Serialize, Deserialize)]
struct Pong {
    #[serde(rename = "type")]
    data: u32,
}

impl ApiError {
    fn new(error: &str) -> Self {
        ApiError {
            error: error.to_string(),
        }
    }
}

#[derive(Clone)]
struct App {
    public_key: String,
}

impl App {
    fn new(public_key: &str) -> Self {
        App {
            public_key: public_key.to_string(),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    env_logger::init();
    
    let app = Router::new()
        .route("/", post(routes::ping_handler))
        .route_layer(axum::middleware::from_fn(routes::mw_sample))
        .with_state(App::new(
            "",
        ));

    run(app).await
}
