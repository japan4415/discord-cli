use serde::Serialize;

/// A builder for constructing API requests.
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct RequestBuilder {
    pub method: Method,
    pub path: String,
    pub query: Vec<(String, String)>,
    pub json_body: Option<serde_json::Value>,
}

#[allow(dead_code)]
#[derive(Debug, Default, Clone)]
pub enum Method {
    #[default]
    Get,
    Post,
    Put,
    Patch,
    Delete,
}

#[allow(dead_code)]
impl RequestBuilder {
    pub fn new(method: Method, path: impl Into<String>) -> Self {
        Self {
            method,
            path: path.into(),
            query: Vec::new(),
            json_body: None,
        }
    }

    pub fn query(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.query.push((key.into(), value.into()));
        self
    }

    pub fn json<T: Serialize>(mut self, body: &T) -> Result<Self, serde_json::Error> {
        self.json_body = Some(serde_json::to_value(body)?);
        Ok(self)
    }
}
