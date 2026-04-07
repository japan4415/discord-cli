pub mod rate_limit;

use anyhow::Result;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE, USER_AGENT};
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::cli::TokenTypeArg;
use crate::error::CliError;

#[derive(Debug, Clone)]
pub enum TokenType {
    Bot,
    User,
}

impl From<TokenTypeArg> for TokenType {
    fn from(arg: TokenTypeArg) -> Self {
        match arg {
            TokenTypeArg::Bot => TokenType::Bot,
            TokenTypeArg::User => TokenType::User,
        }
    }
}

#[derive(Debug, Clone)]
pub struct DiscordClient {
    http: reqwest::Client,
    token: String,
    token_type: TokenType,
    base_url: String,
}

impl DiscordClient {
    pub fn new(token: String, token_type: TokenType) -> Result<Self> {
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert(
            USER_AGENT,
            HeaderValue::from_static("DiscordCLI (https://github.com/discord-cli, 0.1.0)"),
        );

        let http = reqwest::Client::builder()
            .default_headers(headers)
            .build()?;

        Ok(Self {
            http,
            token,
            token_type,
            base_url: "https://discord.com/api/v10".to_string(),
        })
    }

    fn auth_header(&self) -> String {
        match self.token_type {
            TokenType::Bot => format!("Bot {}", self.token),
            TokenType::User => format!("Bearer {}", self.token),
        }
    }

    fn url(&self, path: &str) -> String {
        format!("{}{}", self.base_url, path)
    }

    /// Handle rate limit check and retry logic.
    ///
    /// Returns `Ok(Some(response))` if not rate-limited,
    /// `Ok(None)` after sleeping if rate-limited (caller should retry),
    /// or `Err` if max retries exceeded.
    async fn wait_for_rate_limit(
        &self,
        response: reqwest::Response,
        retry_count: &mut u32,
    ) -> Result<Option<reqwest::Response>> {
        match rate_limit::check_rate_limit(response).await {
            rate_limit::RateLimitResult::Ok(response) => Ok(Some(response)),
            rate_limit::RateLimitResult::RetryAfter(duration) => {
                *retry_count += 1;
                if *retry_count >= rate_limit::MAX_RETRIES {
                    anyhow::bail!(
                        "Rate limit exceeded after {} retries",
                        rate_limit::MAX_RETRIES
                    );
                }
                tracing::warn!(
                    "Rate limited, retry after {:?} (attempt {}/{})",
                    duration,
                    retry_count,
                    rate_limit::MAX_RETRIES
                );
                tokio::time::sleep(duration).await;
                Ok(None)
            }
        }
    }

    pub async fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T> {
        let mut retry_count = 0u32;
        loop {
            let response = self
                .http
                .get(self.url(path))
                .header(AUTHORIZATION, self.auth_header())
                .send()
                .await?;

            if let Some(response) = self.wait_for_rate_limit(response, &mut retry_count).await? {
                return self.handle_response(response).await;
            }
        }
    }

    pub async fn get_with_query<T: DeserializeOwned, Q: Serialize + ?Sized>(
        &self,
        path: &str,
        query: &Q,
    ) -> Result<T> {
        let mut retry_count = 0u32;
        loop {
            let response = self
                .http
                .get(self.url(path))
                .header(AUTHORIZATION, self.auth_header())
                .query(query)
                .send()
                .await?;

            if let Some(response) = self.wait_for_rate_limit(response, &mut retry_count).await? {
                return self.handle_response(response).await;
            }
        }
    }

    pub async fn post<T: DeserializeOwned, B: Serialize>(&self, path: &str, body: &B) -> Result<T> {
        let mut retry_count = 0u32;
        loop {
            let response = self
                .http
                .post(self.url(path))
                .header(AUTHORIZATION, self.auth_header())
                .json(body)
                .send()
                .await?;

            if let Some(response) = self.wait_for_rate_limit(response, &mut retry_count).await? {
                return self.handle_response(response).await;
            }
        }
    }

    #[allow(dead_code)]
    pub async fn post_empty(&self, path: &str) -> Result<()> {
        let mut retry_count = 0u32;
        loop {
            let response = self
                .http
                .post(self.url(path))
                .header(AUTHORIZATION, self.auth_header())
                .send()
                .await?;

            if let Some(response) = self.wait_for_rate_limit(response, &mut retry_count).await? {
                return self.handle_empty_response(response).await;
            }
        }
    }

    pub async fn post_empty_with_body<B: Serialize>(&self, path: &str, body: &B) -> Result<()> {
        let mut retry_count = 0u32;
        loop {
            let response = self
                .http
                .post(self.url(path))
                .header(AUTHORIZATION, self.auth_header())
                .json(body)
                .send()
                .await?;

            if let Some(response) = self.wait_for_rate_limit(response, &mut retry_count).await? {
                return self.handle_empty_response(response).await;
            }
        }
    }

    pub async fn post_multipart<T: DeserializeOwned>(
        &self,
        path: &str,
        form: reqwest::multipart::Form,
    ) -> Result<T> {
        // multipart requests cannot be cloned/retried easily since Form is consumed,
        // so we do not retry on rate limit here. The caller should handle retries if needed.
        let response = self
            .http
            .post(self.url(path))
            .header(AUTHORIZATION, self.auth_header())
            .multipart(form)
            .send()
            .await?;

        self.handle_response(response).await
    }

    pub async fn put<T: DeserializeOwned, B: Serialize>(&self, path: &str, body: &B) -> Result<T> {
        let mut retry_count = 0u32;
        loop {
            let response = self
                .http
                .put(self.url(path))
                .header(AUTHORIZATION, self.auth_header())
                .json(body)
                .send()
                .await?;

            if let Some(response) = self.wait_for_rate_limit(response, &mut retry_count).await? {
                return self.handle_response(response).await;
            }
        }
    }

    pub async fn put_empty(&self, path: &str) -> Result<()> {
        let mut retry_count = 0u32;
        loop {
            let response = self
                .http
                .put(self.url(path))
                .header(AUTHORIZATION, self.auth_header())
                .send()
                .await?;

            if let Some(response) = self.wait_for_rate_limit(response, &mut retry_count).await? {
                return self.handle_empty_response(response).await;
            }
        }
    }

    pub async fn put_empty_with_reason(
        &self,
        path: &str,
        body: &serde_json::Value,
        reason: Option<&str>,
    ) -> Result<()> {
        let mut retry_count = 0u32;
        loop {
            let mut req = self
                .http
                .put(self.url(path))
                .header(AUTHORIZATION, self.auth_header())
                .json(body);

            if let Some(reason) = reason {
                req = req.header(
                    "X-Audit-Log-Reason",
                    urlencoding::encode(reason).into_owned(),
                );
            }

            let response = req.send().await?;

            if let Some(response) = self.wait_for_rate_limit(response, &mut retry_count).await? {
                return self.handle_empty_response(response).await;
            }
        }
    }

    pub async fn patch<T: DeserializeOwned, B: Serialize>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T> {
        let mut retry_count = 0u32;
        loop {
            let response = self
                .http
                .patch(self.url(path))
                .header(AUTHORIZATION, self.auth_header())
                .json(body)
                .send()
                .await?;

            if let Some(response) = self.wait_for_rate_limit(response, &mut retry_count).await? {
                return self.handle_response(response).await;
            }
        }
    }

    pub async fn delete(&self, path: &str) -> Result<()> {
        let mut retry_count = 0u32;
        loop {
            let response = self
                .http
                .delete(self.url(path))
                .header(AUTHORIZATION, self.auth_header())
                .send()
                .await?;

            if let Some(response) = self.wait_for_rate_limit(response, &mut retry_count).await? {
                return self.handle_empty_response(response).await;
            }
        }
    }

    async fn handle_response<T: DeserializeOwned>(&self, response: reqwest::Response) -> Result<T> {
        let status = response.status();
        if !status.is_success() {
            let body = response.text().await.unwrap_or_default();
            return Err(CliError::Api {
                status: status.as_u16(),
                message: body,
            }
            .into());
        }

        let body = response.json::<T>().await?;
        Ok(body)
    }

    async fn handle_empty_response(&self, response: reqwest::Response) -> Result<()> {
        let status = response.status();
        if !status.is_success() {
            let body = response.text().await.unwrap_or_default();
            return Err(CliError::Api {
                status: status.as_u16(),
                message: body,
            }
            .into());
        }

        Ok(())
    }
}
