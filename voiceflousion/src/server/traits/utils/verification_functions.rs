use serde_json::Value;
use crate::errors::{VoiceflousionError, VoiceflousionResult};
#[cfg(feature = "discord_unimplemented")]
use ring::signature::{UnparsedPublicKey, ED25519};

/// Verifies a Discord request using a public key, signature, and timestamp.
///
/// This function checks the authenticity of a Discord request by verifying the provided
/// signature against the public key and a concatenation of the timestamp and request body.
///
/// # Parameters
///
/// * `public_key` - The public key used to verify the request, provided as a hex string.
/// * `signature` - The signature provided by Discord in the request headers, as a hex string.
/// * `timestamp` - The timestamp provided by Discord in the request headers.
/// * `body` - The JSON body of the request.
///
/// # Returns
///
/// A `VoiceflousionResult<()>` indicating success if the verification passes, or an error if it fails.
///
/// # Errors
///
/// Returns a `VoiceflousionError::ClientRequestInvalidBodyError` if the public key or signature
/// are not valid hex strings, or if the verification fails.
///
/// # Example
///
/// ```rust
/// use serde_json::json;
/// use voiceflousion::errors::VoiceflousionError;
/// use voiceflousion::server::traits::utils::discord_public_key_verify;
///
/// let public_key = "abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890";
/// let signature = "1234abcd5678ef901234abcd5678ef901234abcd5678ef901234abcd5678ef90";
/// let timestamp = "1629150245";
/// let body = json!({"key": "value"});
///
/// let result = discord_public_key_verify(&public_key.to_string(), &signature, &timestamp, &body);
///
/// match result {
///     Ok(_) => println!("Verification successful"),
///     Err(err) => eprintln!("Verification failed: {}", err),
/// }
/// ```
#[cfg(feature = "discord_unimplemented")]
pub fn discord_public_key_verify(public_key: &String, signature: &str, timestamp: &str, body: &Value) -> VoiceflousionResult<()>{
    let signature_bytes = hex::decode(signature).map_err(|_| {
        VoiceflousionError::ClientRequestInvalidBodyError(
            "Discord signature".to_string(),
            "Signature isn't a hex string".to_string(),
        )
    })?;

    let public_key_bytes = hex::decode(public_key).map_err(|_| {
        VoiceflousionError::ClientRequestInvalidBodyError(
            "Discord public key".to_string(),
            "Public key isn't a hex string".to_string(),
        )
    })?;
    let timestamp_bytes = timestamp.as_bytes().to_vec();
    let body_bytes = if let Some(string_body) = body.as_str() {
        string_body.as_bytes().to_vec()
    } else if body.is_object() || body.is_array() {
        serde_json::to_vec(body).unwrap_or_else(|_| vec![])
    } else {
        serde_json::to_string(body).map(|s| s.into_bytes()).unwrap_or_else(|_| vec![])
    };

    let message_bytes = [timestamp_bytes, body_bytes].concat();
    let public_key = UnparsedPublicKey::new(&ED25519, &public_key_bytes);

    public_key.verify(&message_bytes, &signature_bytes).map_err(|error|  {
        VoiceflousionError::ClientRequestInvalidBodyError(
            "Discord public key verification".to_string(),
            error.to_string())
    })
}