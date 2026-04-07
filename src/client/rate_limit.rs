use std::time::Duration;

use serde::Deserialize;

/// Maximum number of retries before giving up on rate-limited requests.
pub const MAX_RETRIES: u32 = 5;

#[derive(Deserialize)]
struct RateLimitBody {
    retry_after: Option<f64>,
    #[allow(dead_code)]
    global: Option<bool>,
    #[allow(dead_code)]
    message: Option<String>,
}

/// Result of a rate limit check.
pub enum RateLimitResult {
    /// No rate limit, proceed with the response.
    Ok(reqwest::Response),
    /// Rate limited, wait this duration and retry.
    RetryAfter(Duration),
}

/// Check rate limit from response.
///
/// For 429 responses, reads the JSON body to get `retry_after`.
/// Retry count management is the caller's responsibility.
pub async fn check_rate_limit(response: reqwest::Response) -> RateLimitResult {
    if response.status().as_u16() == 429 {
        // Try to read retry_after from JSON body
        let duration = if let Ok(body) = response.json::<RateLimitBody>().await {
            body.retry_after
                .map(|s| Duration::from_secs_f64(s.max(0.0)))
                .unwrap_or(Duration::from_secs(1))
        } else {
            // Fallback if body parsing fails
            Duration::from_secs(1)
        };
        return RateLimitResult::RetryAfter(duration);
    }

    RateLimitResult::Ok(response)
}
