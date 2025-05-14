use crate::model::channel::Channel;
use crate::model::guild::GuildMember;
use crate::model::message::Message;
use crate::model::primitives::Snowflake;
use crate::model::user::User;
use serde::{Deserialize, Deserializer};
use serde_json::Value;

#[derive(Debug, Deserialize)]
pub struct FullInteraction {
    id: Snowflake,
    application_id: Snowflake,
    #[serde(rename = "type")]
    #[serde(deserialize_with = "InteractionType::deserialize")]
    interaction_type: InteractionType,
    data: Option<Value>,
    guild: Option<Value>,
    guild_id: Option<Snowflake>,
    channel: Option<Channel>,
    channel_id: Option<Snowflake>,
    member: Option<GuildMember>,
    user: Option<User>,
    token: String,
    version: u8,
    message: Option<Message>,
    app_permissions: String,
    locale: Option<String>,
    guild_locale: Option<String>,
    entitlements: Vec<Value>,
    authorizing_integration_owners: Value,
    context: Option<Value>,
    attachment_size_limit: u64,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum Interaction {
    // Ping(PingInteraction),
    // GuildCommand(GuildCommandInteraction),
    Full(FullInteraction),
}

#[derive(Debug, Deserialize)]
pub struct PingInteraction {
    id: Snowflake,
    application_id: Snowflake,
    #[serde(rename = "type")]
    #[serde(deserialize_with = "InteractionType::deserialize")]
    interaction_type: InteractionType,
    user: User,
    token: String,
    version: u8,
    app_permissions: String,
    authorizing_integration_owners: Value,
    attachment_size_limit: u64,
}

#[derive(Debug, Deserialize)]
pub struct GuildCommandInteraction {
    id: Snowflake,
    application_id: Snowflake,
    #[serde(rename = "type")]
    #[serde(deserialize_with = "InteractionType::deserialize")]
    interaction_type: InteractionType,
    data: Option<Value>,
}

#[derive(Debug)]
pub enum InteractionType {
    Ping,
    ApplicationCommand,
    MessageComponent,
    ApplicationCommandAutocomplete,
    ModalSubmit,
}

impl<'de> Deserialize<'de> for InteractionType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: u8 = Deserialize::deserialize(deserializer)?;
        match value {
            1 => Ok(InteractionType::Ping),
            2 => Ok(InteractionType::ApplicationCommand),
            3 => Ok(InteractionType::MessageComponent),
            4 => Ok(InteractionType::ApplicationCommandAutocomplete),
            5 => Ok(InteractionType::ModalSubmit),
            _ => Err(serde::de::Error::custom("Unknown interaction type")),
        }
    }
}
