use serde::{Deserialize, Serialize};

/// Discord Snowflake ID type.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Snowflake(pub String);

impl std::fmt::Display for Snowflake {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for Snowflake {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl From<&str> for Snowflake {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

/// Discord permissions as a bitfield string.
#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Permissions(pub String);

/// Base64-encoded image data for Discord API uploads.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageData(pub String);

/// Display helper for Option fields in Tabled derive.
pub fn display_option<T: std::fmt::Display>(o: &Option<T>) -> String {
    match o {
        Some(v) => v.to_string(),
        None => String::new(),
    }
}

impl ImageData {
    /// Create ImageData from raw bytes and content type (e.g., "image/png").
    pub fn from_bytes(bytes: &[u8], content_type: &str) -> Self {
        use base64::Engine;
        let encoded = base64::engine::general_purpose::STANDARD.encode(bytes);
        Self(format!("data:{};base64,{}", content_type, encoded))
    }
}
