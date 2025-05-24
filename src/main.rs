use crate::util::get_state_proof;
use axum::{
    Router,
    extract::Json,
    extract::rejection::JsonRejection,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::post,
};
use serde::Deserialize;
use serde_json::json;
use tower_http::cors::{Any, CorsLayer};

mod util;

/// Request structure for the state proof endpoint.
#[derive(Debug, Deserialize)]
struct StateProofRequest {
    address: String,
    ethereum_url: String,
    height: u64,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_empty_string_as_none")]
    key: Option<String>,
}

/// Custom deserializer to treat empty strings as None.
fn deserialize_empty_string_as_none<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: Option<String> = Option::deserialize(deserializer)?;
    Ok(s.filter(|s| !s.is_empty()))
}

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/", post(handle_state_proof))
        .layer(cors);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:7777").await.unwrap();
    println!(
        "State proof service listening on {}",
        listener.local_addr().unwrap()
    );
    axum::serve(listener, app).await.unwrap();
}

/// Wrapper handler that logs invalid requests before passing them to the main handler.
async fn handle_state_proof(result: Result<Json<StateProofRequest>, JsonRejection>) -> Response {
    match result {
        Ok(payload) => {
            println!("Request Ok!");
            get_state_proof_handler(payload).await.into_response()
        }
        Err(e) => {
            println!("Invalid request received: {}", e);
            let error_response = json!({
                "status": 400,
                "error": format!("Invalid request format: {}", e),
            });
            (StatusCode::BAD_REQUEST, Json(error_response)).into_response()
        }
    }
}

/// Handler for the state proof endpoint.
async fn get_state_proof_handler(Json(payload): Json<StateProofRequest>) -> impl IntoResponse {
    match get_state_proof(
        &payload.address,
        &payload.ethereum_url,
        payload.height,
        payload.key.as_deref(),
    )
    .await
    {
        Ok(json_bytes) => (
            StatusCode::OK,
            [("Content-Type", "application/json")],
            json_bytes,
        )
            .into_response(),
        Err(e) => {
            let error_response = json!({
                "status": 500,
                "error": format!("Error getting state proof: {}", e)
            });
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)).into_response()
        }
    }
}
