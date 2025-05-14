use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use hex::FromHex;


/// Verifies a Discord request by checking the provided signature against the message.
///
/// # Arguments
///
/// * `public_key_hex` - A string slice containing the public key in hexadecimal format.
/// * `signature_hex` - A string slice containing the signature in hexadecimal format.
/// * `timestamp` - A string slice representing the timestamp of the request.
/// * `body` - A string slice containing the raw body of the request.
///
/// # Returns
///
/// * `Ok(())` - If the signature is valid and matches the message.
/// * `Err(String)` - If the verification fails, with an error message explaining the failure.
///
/// # Errors
///
/// This function returns an error if:
/// - The public key or signature cannot be parsed from hexadecimal.
/// - The public key is invalid.
/// - The signature does not match the message.
pub fn verify_discord_request(
    public_key_hex: &str,
    body_signature: &str,
    timestamp: &str,
    body: &str,
) -> Result<(), String> {

    // Convert public key and signature from hex
    let public_key_bytes =
        <[u8; 32]>::from_hex(public_key_hex).map_err(|_| String::from("Failed to read public key"))?;
    let signature_bytes =
        <[u8; 64]>::from_hex(body_signature).map_err(|_| String::from("Failed to read signature"))?;

    // Reconstruct types
    let verifying_key = VerifyingKey::from_bytes(&public_key_bytes)
        .map_err(|_| String::from("Invalid public key"))?;
    let signature = Signature::from_bytes(&signature_bytes);

    // Message is timestamp and raw body
    let message = [timestamp.as_bytes(), body.as_bytes()].concat();

    // Verify signature
    verifying_key
        .verify(&message, &signature)
        .map_err(|_| String::from("Request body does not match signature"))
}

#[allow(dead_code)]
pub fn generate_signature(signing_key: &SigningKey, body: &str, timestamp: &str) -> Signature {
    let message = [timestamp.as_bytes(), body.as_bytes()].concat();
    signing_key.sign(message.as_slice())
}

#[cfg(test)]
mod tests {
    use ed25519_dalek::SigningKey;
    use rand::rngs::OsRng;
    use crate::security::generate_signature;
    use hex::ToHex;

    #[test]
    fn generate_discord_signature() {
        let body = r#"{"app_permissions":"562949953601536","application_id":"1096551423958855810","attachment_size_limit":524288000,"authorizing_integration_owners":{},"entitlements":[],"id":"0000000001","token":"example_token","type":1,"user":{"avatar":"c6a249645d46209f337279cd2ca998c7","avatar_decoration_data":null,"bot":true,"clan":null,"collectibles":null,"discriminator":"0000","global_name":"Discord","id":"643945264868098049","primary_guild":null,"public_flags":1,"system":true,"username":"discord"},"version":1}"#;
        let timestamp = "1746297636";
        let signing_key = SigningKey::generate(&mut OsRng);
        
        let signature = generate_signature(&signing_key, body, timestamp);
        println!("Verifying Key: {}", signing_key.verifying_key().to_bytes().encode_hex::<String>());
        println!("Signature: {}", signature.to_bytes().encode_hex::<String>());
    }
    
    #[test]
    fn test_verify_discord_request() {
        let signature = "e926a0cba63cfa65588ae268dd055877276d6d4432b2dbb30882156b8fef4fd68706c46c303b9b23a9091a700d8e26e698fd9e979701f10fbf7d358a7adcba07";
        let timestamp = "1746297636";

        let body = r#"{"app_permissions":"562949953601536","application_id":"1096551423958855810","attachment_size_limit":524288000,"authorizing_integration_owners":{},"entitlements":[],"id":"0000000001","token":"example_token","type":1,"user":{"avatar":"c6a249645d46209f337279cd2ca998c7","avatar_decoration_data":null,"bot":true,"clan":null,"collectibles":null,"discriminator":"0000","global_name":"Discord","id":"643945264868098049","primary_guild":null,"public_flags":1,"system":true,"username":"discord"},"version":1}"#;

        let public_key = "f918b1bcd0ada9f101fb43d7c8412e0e9208f4f919a7f0a7e111f063513c3391";
        let result = super::verify_discord_request(public_key, signature, timestamp, body);

        assert!(result.is_ok(), "Signature verification failed: {:?}", result);
    }
}
