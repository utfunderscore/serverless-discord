mod routes;
mod security;

use axum::routing::post;
use axum::Router;
use lambda_http::request::RequestContext::ApiGatewayV1;
use lambda_http::{run, tracing, Error};
use serde::{Deserialize, Serialize};

async fn mw_sample(
    req: axum::extract::Request,
    next: axum::middleware::Next,
) -> impl axum::response::IntoResponse {
    let context = req
        .extensions()
        .get::<lambda_http::request::RequestContext>();
    if let Some(ApiGatewayV1(ctx)) = context {
        tracing::info!("RequestId = {:?}", ctx.request_id);
    }
    next.run(req).await
}

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
        .route_layer(axum::middleware::from_fn(mw_sample))
        .with_state(App::new(
            "9631c9818b210a39d97bb1bab60ff966c566d56066075bca2ae608a6569e427b",
        ));

    run(app).await
}
