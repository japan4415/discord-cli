use anyhow::Result;

use crate::client::DiscordClient;
use crate::models::thread::{ActiveThreadsResponse, ArchivedThreadsResponse, Thread, ThreadMember};

impl DiscordClient {
    pub async fn create_thread(
        &self,
        channel_id: &str,
        params: &serde_json::Value,
    ) -> Result<Thread> {
        self.post(&format!("/channels/{}/threads", channel_id), params)
            .await
    }

    pub async fn join_thread(&self, thread_id: &str) -> Result<()> {
        self.put_empty(&format!("/channels/{}/thread-members/@me", thread_id))
            .await
    }

    pub async fn leave_thread(&self, thread_id: &str) -> Result<()> {
        self.delete(&format!("/channels/{}/thread-members/@me", thread_id))
            .await
    }

    pub async fn list_thread_members(&self, thread_id: &str) -> Result<Vec<ThreadMember>> {
        self.get(&format!("/channels/{}/thread-members", thread_id))
            .await
    }

    pub async fn list_active_threads(&self, guild_id: &str) -> Result<ActiveThreadsResponse> {
        self.get(&format!("/guilds/{}/threads/active", guild_id))
            .await
    }

    pub async fn list_archived_threads(
        &self,
        channel_id: &str,
        archive_type: &str,
    ) -> Result<ArchivedThreadsResponse> {
        self.get(&format!(
            "/channels/{}/threads/archived/{}",
            channel_id, archive_type
        ))
        .await
    }
}
