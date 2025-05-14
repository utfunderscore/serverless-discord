use serde::{Deserialize, Serialize};
use crate::model::primitives::{Snowflake};
use crate::model::user::User;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Channel {
    pub id: Snowflake, // Snowflake
    #[serde(rename = "type")]
    pub channel_type: i32,
    pub guild_id: Option<Snowflake>, // Snowflake
    pub position: Option<i32>,
    pub permission_overwrites: Option<Vec<Overwrite>>,
    pub name: Option<String>,
    pub topic: Option<String>,
    pub nsfw: Option<bool>,
    pub last_message_id: Option<Snowflake>, // Snowflake
    pub bitrate: Option<i32>,
    pub user_limit: Option<i32>,
    pub rate_limit_per_user: Option<i32>,
    pub recipients: Option<Vec<User>>,
    pub icon: Option<String>,
    pub owner_id: Option<Snowflake>, // Snowflake
    pub application_id: Option<Snowflake>, // Snowflake
    pub managed: Option<bool>,
    pub parent_id: Option<Snowflake>, // Snowflake
    pub last_pin_timestamp: Option<String>, // ISO8601 timestamp (String)
    pub rtc_region: Option<String>,
    pub video_quality_mode: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_count: Option<i32>,
    pub member_count: Option<i32>,
    pub thread_metadata: Option<ThreadMetadata>,
    pub member: Option<ThreadMember>,
    pub default_auto_archive_duration: Option<i32>,
    pub permissions: Option<String>,
    pub flags: Option<i32>,
    pub total_message_sent: Option<i32>,
    pub available_tags: Option<Vec<Tag>>,
    pub applied_tags: Option<Vec<Snowflake>>, // Array of Snowflakes (Strings)
    pub default_reaction_emoji: Option<DefaultReaction>,
    pub default_thread_rate_limit_per_user: Option<i32>,
    pub default_sort_order: Option<i32>,
    pub default_forum_layout: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Overwrite {
    pub id: Snowflake, // Snowflake: ID of the role or user
    #[serde(rename = "type")]
    pub overwrite_type: i32, // 0 for role, 1 for member
    pub allow: String,       // Permission bitfield (String representation)
    pub deny: String,        // Permission bitfield (String representation)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ThreadMetadata {
    pub archived: bool, // Whether the thread is archived
    pub auto_archive_duration: i32, // Duration in minutes to automatically archive the thread after recent activity
    pub archive_timestamp: String, // ISO8601 timestamp (String)
    pub locked: bool,     // Whether the thread is locked; when a thread is locked, only users with MANAGE_THREADS permission can unarchive it
    pub invitable: Option<bool>, // Whether non-moderators can add other users to a thread; only available when the thread is private
    pub create_timestamp: Option<String>, // ISO8601 timestamp (String): when the thread was created; only populated for GUILD_PRIVATE_THREAD channels
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ThreadMember {
    pub id: Option<Snowflake>, // Snowflake, may be null: The ID of the thread member
    pub user_id: Option<Snowflake>, // Snowflake, may be null: The ID of the user
    pub join_timestamp: String, // ISO8601 timestamp: the time the user last joined the thread
    pub flags: i32,           // Any user-thread settings, currently only used for notifications
    pub presence: Option<Presence>, //Optional presence for the thread member, if enabled.
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Tag {
    pub id: Snowflake,    // Snowflake: The ID of the tag
    pub name: String,  // The name of the tag (0-20 characters)
    pub moderated: bool, // Whether this tag can only be removed from threads by a member with the MANAGE_THREADS permission
    pub emoji_id: Option<Snowflake>, // Snowflake: The ID of a Discord emoji
    pub emoji_name: Option<String>, // The unicode name of the emoji if it is a custom emoji, null if it is not
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DefaultReaction {
    pub emoji_id: Option<Snowflake>, // Snowflake: The ID of a custom emoji
    pub emoji_name: Option<String>, // The unicode name of the custom emoji, if not a custom emoji.
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Presence {
    pub status: Option<String>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChannelMention {
    pub id: String,       // Snowflake: ID of the channel
    pub guild_id: String, // Snowflake: ID of the guild
    #[serde(rename = "type")]
    pub channel_type: i32, // Type of channel
    pub name: String,     // Name of the channel
}
