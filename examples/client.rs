use anyhow::Result;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<()> {
    // Example 1: Request an account proof
    let account_proof_request = json!({
        "address": "0x07ae8551be970cb1cca11dd7a11f47ae82e70e67",
        "ethereum_url": "https://erigon-tw-rpc.polkachu.com",
        "height": 22545713
    });

    let client = reqwest::Client::new();
    let response = client
        .post("http://localhost:3000/")
        .json(&account_proof_request)
        .send()
        .await?;

    println!("Account Proof Response Status: {}", response.status());
    println!("Account Proof Response Body: {}", response.text().await?);

    // Example 2: Request a storage proof
    let storage_proof_request: serde_json::Value = json!({
        "address": "0xdac17f958d2ee523a2206206994597c13d831ec7",
        "ethereum_url": "https://erigon-tw-rpc.polkachu.com",
        "height": 22545713,
        "key": "0x0000000000000000000000000000000000000000000000000000000000000000"
    });

    let response = client
        .post("http://localhost:3000/")
        .json(&storage_proof_request)
        .send()
        .await?;

    println!("\nStorage Proof Response Status: {}", response.status());
    println!("Storage Proof Response Body: {}", response.text().await?);

    Ok(())
}
