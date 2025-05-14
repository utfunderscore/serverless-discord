use crate::{security, ApiError, App, Pong};
use axum::body::{Body, Bytes};
use axum::extract::State;
use axum::http::{header, HeaderMap};
use axum::response::Response;
use axum::debug_handler;
use std::str::from_utf8;
use lambda_http::request::RequestContext::ApiGatewayV1;
use lambda_http::tracing;
use crate::model::interactions::{FullInteraction, Interaction};

pub async fn mw_sample(
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


fn error_response(message: &str, code: u16) -> Response {
    let error = ApiError::new(message);
    let error_json = serde_json::to_string(&error).unwrap();
    Response::builder()
        .header(header::CONTENT_TYPE, "application/json")
        .status(code)
        .body(Body::from(error_json))
        .unwrap()
}

#[debug_handler]
pub async fn ping_handler(
    key: State<App>,
    req: axum::http::Request<Body>,
) -> Result<Response, Response> {

    println!("Received request: {:?}", req);

    let headers: HeaderMap = req.headers().clone();
    let body: Body = req.into_body();
    let body_bytes = axum::body::to_bytes(body, usize::MAX)
        .await
        .map_err(|_| error_response("Failed to read body", 500))?;

    if let Err(e) = validate_request(&headers, &body_bytes, key.public_key.clone()) {
        println!("Received invalid signature: {}", e);
        return Err(error_response("Invalid request", 401))
    }

    let json_value_result = serde_json::from_slice::<Interaction>(&body_bytes);
    if let Err(e) = json_value_result {
        eprintln!("Failed to parse json: {}", e);
        return Err(error_response("Invalid request", 400))
    }
    
    let interaction = json_value_result.unwrap();
    println!("Received interaction: {:#?}", interaction);

    Ok(
        Response::builder()
            .header(header::CONTENT_TYPE, "application/json")
            .status(200)
            .body(Body::from(serde_json::to_string(&Pong { data: 1 }).unwrap()))
            .unwrap(),
    )
}

fn validate_request(headers: &HeaderMap, body_bytes: &Bytes, public_key: String) -> Result<(), String> {
    
    let signature = headers
        .get("x-signature-ed25519")
        .ok_or("Invalid signature")?;
    let timestamp = headers
        .get("x-signature-timestamp")
        .ok_or("Invalid timestamp")?;



    let key = public_key.as_str();

    let body_str = from_utf8(body_bytes.as_ref()).map_err(|_| "Failed to convert body to string")?;

    security::verify_discord_request(
        key,
        signature.to_str().unwrap(),
        timestamp.to_str().unwrap(),
        body_str,
    )
}
