use std::time::{SystemTime, UNIX_EPOCH};

/// Generate a unique ID using onchain randomness
pub async fn generate_id() -> String {
    // Get 32 random bytes from the IC management canister
    let random_bytes = ic_cdk::management_canister::raw_rand().await.unwrap_or_else(|_| {
        // Fallback to timestamp if randomness fails
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        timestamp.to_le_bytes().to_vec()
    });
    
    // Convert to hex string
    hex::encode(random_bytes)
} 