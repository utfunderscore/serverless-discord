use ed25519_dalek::{Signature, Verifier, VerifyingKey};
use hex::FromHex;

/// Verifies that the request came from discord
/// public_key_hex: The public key of the discord application
/// signature_hex: The signature of the request
/// timestamp: The timestamp of the request
/// body: The body of the request
pub fn verify_discord_request(
    public_key_hex: &str,
    signature_hex: &str,
    timestamp: &str,
    body: &str,
) -> Result<(), String> {
    // Convert public key and signature from hex
    let public_key_bytes =
        <[u8; 32]>::from_hex(public_key_hex).map_err(|_| String::from("Invalid public key"))?;
    let signature_bytes =
        <[u8; 64]>::from_hex(signature_hex).map_err(|_| String::from("Invalid signature"))?;

    // Reconstruct types
    let verifying_key = VerifyingKey::from_bytes(&public_key_bytes)
        .map_err(|_| String::from("Invalid public key"))?;
    let signature = Signature::from_bytes(&signature_bytes);

    // Message is timestamp and raw body
    let message = [timestamp.as_bytes(), body.as_bytes()].concat();

    // Verify signature
    verifying_key
        .verify(&message, &signature)
        .map_err(|_| String::from("Invalid signature"))
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_verify_discord_request() {

    }
}
