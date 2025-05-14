use serde::{Deserialize, Serialize};
use crate::model::primitives::Snowflake;
use crate::model::user::User;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Guild {
    pub id: Snowflake,                   // Guild ID
    pub name: String,                      // Guild name (2-100 characters)
    pub icon: Option<String>,             // Icon hash
    pub icon_hash: Option<String>,        // Icon hash, returned when in the template object
    pub splash: Option<String>,            // Splash hash
    pub discovery_splash: Option<String>, // Discovery splash hash; only present for guilds with the "DISCOVERABLE" feature
    pub owner_id: Snowflake,               // User ID of owner
    pub permissions: Option<String>,       // Total permissions for the user in the guild (excludes channel overrides)
    pub region: Option<String>,           // Voice region ID
    pub afk_channel_id: Option<Snowflake>, // ID of AFK channel
    pub afk_timeout: i32,                  // AFK timeout in seconds
    pub verification_level: i32,         // Verification level required for the guild
    pub default_message_notifications: i32, // Default message notifications level
    pub explicit_content_filter: i32,      // Explicit content filter level
    pub roles: Vec<Role>,               // Roles in the guild
    pub emojis: Vec<Emoji>,             // Custom guild emojis
    pub features: Vec<String>, // Enabled guild features
    pub mfa_level: i32,        // Required MFA level for the guild
    pub application_id: Option<Snowflake>, // Application ID of the guild creator if it is bot-created
    pub system_channel_id: Option<Snowflake>, // The ID of the channel where guild notices such as welcome messages and boost events are posted
    pub system_channel_flags: i32,         // System channel flags
    pub rules_channel_id: Option<Snowflake>,   // The ID of the channel where community guidelines/rules are posted
    pub max_presences: Option<i32>,           // Max number of presences for the guild (null is always returned, apart from the largest of guilds)
    pub max_members: Option<i32>,             // The maximum number of members for the guild
    pub vanity_url_code: Option<String>,       // The vanity URL code for the guild
    pub description: Option<String>,           // The description for the guild
    pub banner: Option<String>,                // Banner hash
    pub premium_tier: i32,                     // Premium tier (Server Boost level)
    pub premium_subscription_count: Option<i32>, // The number of boosts this guild currently has
    pub preferred_locale: String,              // The preferred locale of this guild for guild events and discovery
    pub public_updates_channel_id: Option<Snowflake>, // The ID of the channel where admins and moderators of Community guilds receive notices from Discord
    pub stage_instances: Option<Vec<StageInstance>>, // Stage instances in the guild
    pub approximate_member_count: Option<i32>, // the approximate count of members in this guild, returned from the GET /guilds/<id> endpoint when with_counts is true
    pub approximate_presence_count: Option<i32>, // the approximate count of non-offline members in this guild, returned from the GET /guilds/<id> endpoint when with_counts is true
    pub welcome_screen: Option<WelcomeScreen>, // the welcome screen of a Community guild, enabled through the Community Home
    pub nsfw_level: i32, //Guild NSFW level
    pub stickers: Option<Vec<Sticker>>, //Custom guild stickers
    pub premium_progress_bar_enabled: bool, //Whether the guild has the boost progress bar enabled
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GuildMember {
    pub user: Option<User>,            // The user this guild member represents
    pub nick: Option<String>,          // This user's guild nickname
    pub avatar: Option<String>,        // The member's guild avatar hash
    pub roles: Vec<String>,            // Array of role object IDs
    pub joined_at: String,             // When the user joined the guild
    pub premium_since: Option<String>, // When the user started boosting the guild
    pub deaf: bool,                    // Whether the user is deafened in voice channels
    pub mute: bool,                    // Whether the user is muted in voice channels
    pub pending: Option<bool>, // Whether the user has not yet passed the guild's Membership Screening requirements
    pub permissions: Option<String>, // Total permissions of the member in the channel, including overwrites, returned when in the interaction object
    pub communication_disabled_until: Option<String>, // When the user's timeout will expire and the user will be able to communicate in the guild again
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Role {
    pub id: Snowflake,
    pub name: String,
    pub color: Option<i32>,
    pub hoist: bool,
    pub icon: Option<String>,
    pub unicode_emoji: Option<String>,
    pub position: i32,
    pub permissions: String,
    pub managed: bool,
    pub mentionable: bool,
    pub tags: Option<RoleTags>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RoleTags {
    pub bot_id: Option<Snowflake>,
    pub integration_id: Option<Snowflake>,
    pub premium_subscriber: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Emoji {
    pub id: Option<Snowflake>,
    pub name: String,
    pub roles: Option<Vec<Snowflake>>,
    pub require_colons: Option<bool>,
    pub managed: Option<bool>,
    pub animated: Option<bool>,
    pub available: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StageInstance {
    pub id: Snowflake,
    pub guild_id: Snowflake,
    pub channel_id: Snowflake,
    pub topic: String,
    pub privacy_level: i32,
    pub discoverable_disabled: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WelcomeScreen {
    pub description: Option<String>,
    pub welcome_channels: Vec<WelcomeChannel>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WelcomeChannel {
    pub channel_id: Snowflake,
    pub description: String,
    pub emoji_id: Option<Snowflake>,
    pub emoji_name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Sticker {
    pub id: Snowflake,
    pub name: String,
    pub description: Option<String>,
    pub tags: Option<String>,
    pub asset: String,
    pub format_type: i32,
    pub available: Option<bool>,
    pub guild_id: Option<Snowflake>,
    pub user: Option<Snowflake>, // Assuming the User struct is in a parent module
    pub sort_value: Option<i32>,
}

