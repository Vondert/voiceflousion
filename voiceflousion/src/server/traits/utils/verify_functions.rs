use serde_json::Value;
use crate::errors::{VoiceflousionError, VoiceflousionResult};
#[cfg(feature = "discord_unimplemented")]
use ring::signature::{UnparsedPublicKey, ED25519};

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