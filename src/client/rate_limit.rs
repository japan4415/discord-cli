use std::time::Duration;

/// Check if the response indicates rate limiting and return the duration to wait.
pub fn check_rate_limit(response: &reqwest::Response) -> Option<Duration> {
    if response.status().as_u16() == 429 {
        if let Some(retry_after) = response.headers().get("retry-after") {
            if let Ok(secs) = retry_after.to_str().unwrap_or("1").parse::<f64>() {
                return Some(Duration::from_secs_f64(secs));
            }
        }
        return Some(Duration::from_secs(1));
    }

    if let Some(remaining) = response.headers().get("x-ratelimit-remaining") {
        if remaining.to_str().unwrap_or("1") == "0" {
            if let Some(reset_after) = response.headers().get("x-ratelimit-reset-after") {
                if let Ok(secs) = reset_after.to_str().unwrap_or("1").parse::<f64>() {
                    return Some(Duration::from_secs_f64(secs));
                }
            }
        }
    }

    None
}
