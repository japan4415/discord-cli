use std::time::Duration;

use serde::Deserialize;

/// Maximum number of retries before giving up on rate-limited requests.
pub const MAX_RETRIES: u32 = 5;

/// Minimum retry duration floor (100ms) to prevent zero or negative wait times.
const MIN_RETRY_DURATION: f64 = 0.1;
/// Minimum retry duration in milliseconds, used as the default fallback.
const MIN_RETRY_DURATION_MS: u64 = 100;

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
/// Falls back to the `Retry-After` HTTP header if JSON parsing fails
/// (e.g., Cloudflare edge returning non-JSON 429 responses).
/// Retry count management is the caller's responsibility.
pub async fn check_rate_limit(response: reqwest::Response) -> RateLimitResult {
    if response.status().as_u16() == 429 {
        // Save header value before consuming body with json()
        let header_retry_after = response
            .headers()
            .get("retry-after")
            .and_then(|v| v.to_str().ok())
            .and_then(|s| s.parse::<f64>().ok());

        // Try JSON body first, fall back to header, then default
        let duration = if let Ok(body) = response.json::<RateLimitBody>().await {
            body.retry_after
                .or(header_retry_after)
                .map(|s| Duration::from_secs_f64(s.max(MIN_RETRY_DURATION)))
                .unwrap_or(Duration::from_millis(MIN_RETRY_DURATION_MS))
        } else {
            header_retry_after
                .map(|s| Duration::from_secs_f64(s.max(MIN_RETRY_DURATION)))
                .unwrap_or(Duration::from_millis(MIN_RETRY_DURATION_MS))
        };
        return RateLimitResult::RetryAfter(duration);
    }

    RateLimitResult::Ok(response)
}
