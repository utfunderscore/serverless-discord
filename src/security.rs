use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use hex::FromHex;
use hex::ToHex;
use rand::rngs::OsRng;


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

#[cfg(test)]
mod tests {

    #[test]
    fn test_verify_discord_request() {
        let signature = "bee13572331740d7b68d81fd42c0c3a90c74a34648690f59088235607109397d1af696d24623eaca20e4c98ed375874ff99af491819727581caed6902bbb300f";
        let timestamp = "1746297636";

        let body = r#"{"app_permissions":"562949953601536","application_id":"1096551423958855810","attachment_size_limit":524288000,"authorizing_integration_owners":{},"entitlements":[],"id":"0000000001","token":"example_token","type":1,"user":{"avatar":"c6a249645d46209f337279cd2ca998c7","avatar_decoration_data":null,"bot":true,"clan":null,"collectibles":null,"discriminator":"0000","global_name":"Discord","id":"643945264868098049","primary_guild":null,"public_flags":1,"system":true,"username":"discord"},"version":1}"#;

        let public_key = "31772c1bf4464dad0905b4f12cfb02ed1628a7ff32e84f547b6938fbacd4a5fc";
        let result = super::verify_discord_request(public_key, signature, timestamp, body);

    }
}
