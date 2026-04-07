use anyhow::Result;

use crate::client::DiscordClient;
use crate::models::member::GuildMember;

impl DiscordClient {
    pub async fn get_member(&self, guild_id: &str, user_id: &str) -> Result<GuildMember> {
        self.get(&format!("/guilds/{}/members/{}", guild_id, user_id))
            .await
    }

    pub async fn list_members(
        &self,
        guild_id: &str,
        limit: Option<u64>,
    ) -> Result<Vec<GuildMember>> {
        let mut query: Vec<(&str, String)> = Vec::new();
        if let Some(l) = limit {
            query.push(("limit", l.to_string()));
        }
        self.get_with_query(&format!("/guilds/{}/members", guild_id), &query)
            .await
    }

    pub async fn search_members(
        &self,
        guild_id: &str,
        query_str: &str,
        limit: Option<u64>,
    ) -> Result<Vec<GuildMember>> {
        let mut query: Vec<(&str, String)> = vec![("query", query_str.to_string())];
        if let Some(l) = limit {
            query.push(("limit", l.to_string()));
        }
        self.get_with_query(&format!("/guilds/{}/members/search", guild_id), &query)
            .await
    }

    pub async fn kick_member(&self, guild_id: &str, user_id: &str) -> Result<()> {
        self.delete(&format!("/guilds/{}/members/{}", guild_id, user_id))
            .await
    }

    pub async fn edit_member(
        &self,
        guild_id: &str,
        user_id: &str,
        params: &serde_json::Value,
    ) -> Result<GuildMember> {
        self.patch(&format!("/guilds/{}/members/{}", guild_id, user_id), params)
            .await
    }
}
