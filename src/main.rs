use crate::util::get_state_proof;
use axum::extract::rejection::JsonRejection;
use axum::{
    Router, extract::Json, http::StatusCode, response::IntoResponse, response::Response,
    routing::post,
};
use serde::Deserialize;
use tower_http::cors::{Any, CorsLayer};

mod util;

/// Request structure for the state proof endpoint.
///
/// This structure represents the input parameters required to generate an Ethereum state proof.
/// It supports both account proofs and storage proofs depending on whether a storage key is provided.
///
/// # Fields
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

/// Custom deserializer that converts empty strings to None.
///
/// This is used to handle cases where the storage key is provided as an empty string,
/// which should be treated as if no key was provided.
///
/// # Arguments
/// * `deserializer` - The serde deserializer
///
/// # Returns
/// * `Result<Option<String>, D::Error>` - None if the string is empty, Some(string) otherwise
fn deserialize_empty_string_as_none<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: Option<String> = Option::deserialize(deserializer)?;
    Ok(s.filter(|s| !s.is_empty()))
}

#[tokio::main]
async fn main() {
    // Configure CORS to allow requests from any origin
    // This is useful for development and when the API is called from web applications
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Create the API router with a single endpoint for state proofs
    let app = Router::new()
        .route("/", post(handle_state_proof))
        .layer(cors);

    // Start the server on localhost:3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!(
        "State proof service listening on {}",
        listener.local_addr().unwrap()
    );
    axum::serve(listener, app).await.unwrap();
}

/// Wrapper handler that logs invalid requests before passing them to the main handler
async fn handle_state_proof(result: Result<Json<StateProofRequest>, JsonRejection>) -> Response {
    match result {
        Ok(payload) => {
            println!("Request Ok!");
            get_state_proof_handler(payload).await.into_response()
        }
        Err(e) => {
            println!("Invalid request received: {}", e);
            (
                StatusCode::BAD_REQUEST,
                format!("Invalid request format: {}", e),
            )
                .into_response()
        }
    }
}

/// Handler for the state proof endpoint.
///
/// This handler processes incoming state proof requests and returns either a valid proof
/// or an error response. It supports both account proofs and storage proofs.
///
/// # Arguments
/// * `payload` - The deserialized StateProofRequest containing the proof parameters
///
/// # Returns
/// * `impl IntoResponse` - Either:
///   * 200 OK with the StateProofResponse if successful
///   * 500 Internal Server Error with error message if the proof generation fails
///
/// # Errors
/// The handler will return a 500 error if:
/// * The Ethereum RPC request fails
/// * The proof generation fails
/// * The proof serialization fails
async fn get_state_proof_handler(Json(payload): Json<StateProofRequest>) -> impl IntoResponse {
    match get_state_proof(
        &payload.address,
        &payload.ethereum_url,
        payload.height,
        payload.key.as_deref(),
    )
    .await
    {
        Ok(proof) => (StatusCode::OK, Json(proof)).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error getting state proof: {}", e),
        )
            .into_response(),
    }
}
