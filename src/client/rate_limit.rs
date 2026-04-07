use std::time::Duration;

use serde::Deserialize;

/// Maximum number of retries before giving up on rate-limited requests.
pub const MAX_RETRIES: u32 = 5;

/// Minimum duration to wait before retrying a rate-limited request.
const MIN_RETRY_WAIT: Duration = Duration::from_millis(100);

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

/// Parse `Retry-After` header value, supporting both seconds (numeric) and
/// HTTP-date (RFC 2822) formats.
fn parse_retry_after_header(value: &str) -> Option<f64> {
    // Try seconds format first
    value.parse::<f64>().ok().or_else(|| {
        // Try HTTP-date format (RFC 2822 / RFC 7231)
        chrono::DateTime::parse_from_rfc2822(value).ok().map(|dt| {
            let now = chrono::Utc::now();
            let diff = dt.signed_duration_since(now);
            diff.num_milliseconds().max(0) as f64 / 1000.0
        })
    })
}

/// Check rate limit from response.
///
/// For 429 responses, reads the `Retry-After` header (seconds or HTTP-date)
/// and falls back to the JSON body `retry_after` field.
/// Retry count management is the caller's responsibility.
pub async fn check_rate_limit(response: reqwest::Response) -> RateLimitResult {
    if response.status().as_u16() == 429 {
        // Try Retry-After header first (supports both seconds and HTTP-date)
        let header_retry_after: Option<f64> = response
            .headers()
            .get("retry-after")
            .and_then(|v| v.to_str().ok())
            .and_then(parse_retry_after_header);

        // Fall back to JSON body retry_after
        let duration = if let Some(secs) = header_retry_after {
            Duration::from_secs_f64(secs).max(MIN_RETRY_WAIT)
        } else if let Ok(body) = response.json::<RateLimitBody>().await {
            body.retry_after
                .map(|s| Duration::from_secs_f64(s).max(MIN_RETRY_WAIT))
                .unwrap_or(Duration::from_secs(1))
        } else {
            // Fallback if both header and body parsing fail
            Duration::from_secs(1)
        };
        return RateLimitResult::RetryAfter(duration);
    }

    RateLimitResult::Ok(response)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_retry_after_seconds() {
        let result = parse_retry_after_header("1.5");
        assert_eq!(result, Some(1.5));
    }

    #[test]
    fn test_parse_retry_after_integer_seconds() {
        let result = parse_retry_after_header("3");
        assert_eq!(result, Some(3.0));
    }

    #[test]
    fn test_parse_retry_after_invalid() {
        let result = parse_retry_after_header("not-a-number-or-date");
        assert_eq!(result, None);
    }

    #[test]
    fn test_parse_retry_after_http_date() {
        // RFC 2822 date format
        let result = parse_retry_after_header("Wed, 08 Apr 2026 12:00:00 GMT");
        // Should return Some with a non-negative value (the exact value depends on current time)
        assert!(result.is_some());
        assert!(result.unwrap() >= 0.0);
    }

    #[test]
    fn test_parse_retry_after_past_date() {
        // A date in the past should return 0.0 (clamped to non-negative)
        let result = parse_retry_after_header("Mon, 01 Jan 2024 00:00:00 GMT");
        assert!(result.is_some());
        assert_eq!(result.unwrap(), 0.0);
    }

    #[test]
    fn test_min_retry_wait() {
        // Duration from a very small retry_after should be clamped to MIN_RETRY_WAIT
        let d = Duration::from_secs_f64(0.001).max(MIN_RETRY_WAIT);
        assert_eq!(d, MIN_RETRY_WAIT);
    }
}
