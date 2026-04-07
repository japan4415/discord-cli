use serde::{Deserialize, Serialize};

/// Base64-encoded image data for Discord API uploads.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageData(pub String);

/// Display helper for Option fields in Tabled derive.
pub fn display_option<T: std::fmt::Display>(o: &Option<T>) -> String {
    match o {
        Some(v) => v.to_string(),
        None => "-".to_string(),
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
