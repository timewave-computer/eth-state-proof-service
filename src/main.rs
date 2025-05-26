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
///
/// This struct represents the JSON payload expected by the state proof endpoint.
/// All fields are required except for `key`, which is optional.
///
/// # Fields
///
/// * `address` - The Ethereum address to get the proof for (hex string, 0x-prefixed)
/// * `ethereum_url` - The RPC URL for the Ethereum node (e.g., Infura, Alchemy)
/// * `height` - The block height/number to get the proof for
/// * `key` - Optional storage slot key for storage proofs (hex string, 0x-prefixed)
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
///
/// This function is used to deserialize optional string fields in the request.
/// If the field is an empty string, it will be converted to None.
///
/// # Arguments
///
/// * `deserializer` - The Serde deserializer
///
/// # Returns
///
/// Returns `Ok(None)` for empty strings, `Ok(Some(String))` for non-empty strings,
/// or a deserialization error if the input is invalid.
fn deserialize_empty_string_as_none<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: Option<String> = Option::deserialize(deserializer)?;
    Ok(s.filter(|s| !s.is_empty()))
}

/// Main entry point for the application.
///
/// This function:
/// 1. Sets up CORS middleware to allow cross-origin requests
/// 2. Creates the router with the state proof endpoint
/// 3. Binds to port 7777 on all interfaces
/// 4. Starts the Axum server
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
///
/// This function:
/// 1. Logs successful requests
/// 2. Logs and formats error responses for invalid requests
/// 3. Delegates valid requests to the main handler
///
/// # Arguments
///
/// * `result` - The result of deserializing the request body
///
/// # Returns
///
/// Returns an Axum response containing either:
/// * The state proof for valid requests
/// * An error message for invalid requests
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
///
/// This function:
/// 1. Extracts the request parameters
/// 2. Calls the state proof generation function
/// 3. Returns either the proof or an error response
///
/// # Arguments
///
/// * `payload` - The validated request payload
///
/// # Returns
///
/// Returns an Axum response containing either:
/// * The state proof bytes for successful requests
/// * An error message for failed requests
use axum::body::Body;
use axum::http::Response as HttpResponse;

async fn get_state_proof_handler(Json(payload): Json<StateProofRequest>) -> impl IntoResponse {
    match get_state_proof(
        &payload.address,
        &payload.ethereum_url,
        payload.height,
        payload.key.as_deref(),
    )
    .await
    {
        Ok(json_bytes) => HttpResponse::builder()
            .status(StatusCode::OK)
            .body(Body::from(json_bytes))
            .unwrap()
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
