use ethereum_merkle_proofs::{
    ethereum_rpc::rpc::EvmMerkleRpcClient,
    merkle_lib::types::{EthereumProofType, EthereumSimpleProof},
};
use valence_coprocessor::StateProof;

/// Retrieves an Ethereum state proof for a given address and block height.
///
/// This function generates either an account proof or a storage proof depending on whether
/// a storage key is provided. The proof can be used to verify the state of an Ethereum
/// account or a specific storage slot at a given block height.
///
/// # Arguments
///
/// * `address` - The Ethereum address to get the proof for (hex string, 0x-prefixed)
/// * `ethereum_url` - The RPC URL for the Ethereum node (e.g., Infura, Alchemy)
/// * `height` - The block height/number to get the proof for
/// * `key` - Optional storage slot key for storage proofs (hex string, 0x-prefixed)
///
/// # Returns
///
/// Returns a `StateProof` containing:
/// * `domain` - Always set to "ethereum"
/// * `root` - The Merkle root (currently set to zero, TODO: implement)
/// * `payload` - Additional data (currently empty)
/// * `proof` - The serialized proof bytes containing either:
///   * An account proof - when no storage key is provided
///   * A storage proof - when a storage key is provided
///
/// # Errors
///
/// Returns an error if:
/// * The Ethereum RPC request fails
/// * The proof generation fails
/// * The proof serialization fails
///
/// # Example
///
/// ```rust
/// let proof = get_state_proof(
///     "0x1234...",
///     "https://eth-mainnet.alchemyapi.io/v2/your-api-key",
///     12345678,
///     None
/// ).await?;
/// ```
pub async fn get_state_proof(
    address: &str,
    ethereum_url: &str,
    height: u64,
    key: Option<&str>,
) -> anyhow::Result<StateProof> {
    let merkle_prover = EvmMerkleRpcClient {
        rpc_url: ethereum_url.to_string(),
    };

    // Generate either a storage proof (if key is provided) or an account proof
    match key {
        Some(key) => {
            // Request a storage proof for the specified key
            let combined_proof = merkle_prover
                .get_account_and_storage_proof(key, address, height)
                .await
                .map_err(|e| anyhow::anyhow!("Failed to get storage proof: {}", e))?;
            println!("Combined proof: {:?}", combined_proof);
            let simple_proof = EthereumSimpleProof::from_combined_proof(combined_proof);
            let proof = EthereumProofType::Simple(simple_proof);
            let proof_bytes = serde_json::to_vec(&proof)?;
            Ok(StateProof {
                domain: "ethereum".to_string(),
                // TODO: Implement getting the actual block root
                // This requires fetching the block header for the specified height
                root: [0; 32],
                payload: Vec::new(),
                proof: proof_bytes,
            })
        }
        None => {
            // Request an account proof
            let account_proof = merkle_prover.get_account_proof(address, height).await;
            let proof = EthereumProofType::Account(account_proof.unwrap());
            let proof_bytes = serde_json::to_vec(&proof)?;
            Ok(StateProof {
                domain: "ethereum".to_string(),
                root: [0; 32],
                payload: Vec::new(),
                proof: proof_bytes,
            })
        }
    }
}
