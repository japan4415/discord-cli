use serde::{Deserialize, Serialize};
use tabled::Tabled;

use super::common::display_option;

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize, Tabled)]
pub struct Poll {
    #[tabled(skip)]
    pub question: Option<serde_json::Value>,
    #[tabled(skip)]
    pub answers: Option<Vec<PollAnswer>>,
    #[tabled(display_with = "display_option")]
    pub expiry: Option<String>,
    #[tabled(display_with = "display_option")]
    pub allow_multiselect: Option<bool>,
    #[tabled(skip)]
    pub results: Option<serde_json::Value>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize, Tabled)]
pub struct PollAnswer {
    #[tabled(display_with = "display_option")]
    pub answer_id: Option<u64>,
    #[tabled(skip)]
    pub poll_media: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Tabled)]
pub struct PollVoter {
    pub id: String,
    pub username: String,
    #[tabled(display_with = "display_option")]
    pub discriminator: Option<String>,
    #[tabled(display_with = "display_option")]
    pub avatar: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PollVotersResponse {
    pub users: Vec<PollVoter>,
}
