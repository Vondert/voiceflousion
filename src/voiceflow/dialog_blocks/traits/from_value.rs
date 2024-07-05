use serde_json::Value;

pub(crate) trait FromValue: Sized {
    type Error;
    fn from_value(value: &Value) -> Result<Self, Self::Error>;
}