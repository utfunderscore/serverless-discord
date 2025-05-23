use crate::model::channel::{Channel, ChannelMention};
use crate::model::user::User;
use serde::{Deserialize, Serialize};
use crate::model::guild::GuildMember;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    pub id: String,                                    // Snowflake: The ID of the message
    pub channel_id: String, // Snowflake: The ID of the channel the message was sent in
    pub guild_id: Option<String>, // Snowflake: The ID of the guild the message was sent in (may not be present)
    pub author: User, // The author of this message (not guaranteed to be a valid user object, e.g., webhook messages)
    pub content: String, // The message contents (up to 2000 characters)
    pub timestamp: String, // ISO8601 timestamp: When this message was sent
    pub edited_timestamp: Option<String>, // ISO8601 timestamp: When this message was edited (or null if never)
    pub tts: bool,                        // Whether this was a TTS message
    pub mention_everyone: bool,           // Whether this message mentions everyone
    pub mentions: Vec<User>,              // All user mentions in the message (User objects)
    pub mention_roles: Vec<String>,       // All role mentions in the message (Snowflake IDs)
    pub mention_channels: Option<Vec<ChannelMention>>, // All channel mentions in the message (ChannelMention objects)
    pub attachments: Vec<Attachment>,                  // Any attached files
    pub embeds: Vec<Embed>,                            // Any embedded content
    pub reactions: Option<Vec<Reaction>>,              // Reactions to the message (if any)
    pub nonce: Option<String>, // Used for validating messages, typically non-existent (string or integer)
    pub pinned: bool,          // Whether this message is pinned
    pub webhook_id: Option<String>, // Snowflake: If the message is generated by a webhook, this is the webhook's ID
    #[serde(rename = "type")]
    pub message_type: i32, // Message type
    pub activity: Option<MessageActivity>, // Message activity object (for example, invites and rich presence)
    pub application: Option<MessageApplication>, // Message application object (for interactions)
    pub application_id: Option<String>, // Snowflake: If the message is an interaction or application command, this is the application's ID
    pub message_reference: Option<MessageReference>, // Message reference object (for crossposted messages, channel follow add messages, pin messages, and replied messages)
    pub flags: Option<i32>,                          // Message flags combined as a bitfield
    pub referenced_message: Option<Box<Message>>, // The message associated with the message_reference
    pub interaction: Option<MessageInteraction>,  // Message interaction object
    pub thread: Option<Channel>, // The thread that was started from this message, includes thread member object
    pub components: Option<Vec<Component>>, // Message components
    pub sticker_items: Option<Vec<StickerItem>>, // Message sticker item objects
    pub position: Option<i32>,   // The position of the message in the channel
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Embed {
    pub title: Option<String>, // Title of embed
    #[serde(rename = "type")]
    pub embed_type: Option<String>, // Type of embed (always "rich" for webhook embeds)
    pub description: Option<String>, // Description of embed
    pub url: Option<String>,   // URL of embed
    pub timestamp: Option<String>, // ISO8601 timestamp: timestamp of embed content
    pub color: Option<i32>,    // Color code of embed
    pub footer: Option<EmbedFooter>, // Embed footer object
    pub image: Option<EmbedImage>, // Embed image object
    pub thumbnail: Option<EmbedThumbnail>, // Embed thumbnail object
    pub video: Option<EmbedVideo>, // Embed video object
    pub provider: Option<EmbedProvider>, // Embed provider object
    pub author: Option<EmbedAuthor>, // Embed author object
    pub fields: Option<Vec<EmbedField>>, // Array of embed field objects
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EmbedProvider {
    pub name: Option<String>, // Name of provider
    pub url: Option<String>,  // URL of provider
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EmbedAuthor {
    pub name: Option<String>,           // Name of author
    pub url: Option<String>,            // URL of author
    pub icon_url: Option<String>, // URL of author icon (only supports http(s) and discord.com)
    pub proxy_icon_url: Option<String>, // A proxied URL of author icon
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EmbedField {
    pub name: String,         // Name of field
    pub value: String,        // Value of field
    pub inline: Option<bool>, // Whether this field should display inline
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EmbedFooter {
    pub text: String,                   // Footer text
    pub icon_url: Option<String>, // URL of footer icon (only supports http(s) and discord.com)
    pub proxy_icon_url: Option<String>, // A proxied URL of footer icon
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EmbedImage {
    pub url: Option<String>, // Source URL of image (only supports http(s) and discord.com)
    pub proxy_url: Option<String>, // A proxied URL of the image
    pub height: Option<i32>, // Height of image
    pub width: Option<i32>,  // Width of image
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EmbedThumbnail {
    pub url: Option<String>, // Source URL of thumbnail (only supports http(s) and discord.com)
    pub proxy_url: Option<String>, // A proxied URL of the thumbnail
    pub height: Option<i32>, // Height of thumbnail
    pub width: Option<i32>,  // Width of thumbnail
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EmbedVideo {
    pub url: Option<String>, // Source URL of video
    pub height: Option<i32>, // Height of video
    pub width: Option<i32>,  // Width of video
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessageActivity {
    #[serde(rename = "type")]
    pub activity_type: i32, // Message activity type
    pub party_id: Option<String>, // party_id from a Rich Presence event
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessageApplication {
    pub id: String,                  // Snowflake: The ID of the application
    pub cover_image: Option<String>, // The ID of the embed's image asset
    pub description: String,         // The application's description
    pub icon: Option<String>,        // The ID of the application's icon
    pub name: String,                // The name of the application
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessageReference {
    pub message_id: Option<String>, // Snowflake: ID of the originating message
    pub channel_id: Option<String>, // Snowflake: ID of the originating message's channel
    pub guild_id: Option<String>,   // Snowflake: ID of the originating message's guild
    pub fail_if_not_exists: Option<bool>, // When sending, whether to error if the referenced message doesn't exist instead of sending as a normal (non-reply) message, default true
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessageInteraction {
    pub id: String, // Snowflake: The ID of the interaction
    #[serde(rename = "type")]
    pub interaction_type: i32, // The type of interaction
    pub name: String, // The name of the application command
    pub user: User, // The user who invoked the interaction
    pub member: Option<GuildMember>, // The member who invoked the interaction in the guild
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Reaction {
    pub count: i32,           // Number of times this emoji has been used to react
    pub emoji: ReactionEmoji, // Reaction emoji object
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ReactionEmoji {
    pub id: Option<String>, // Snowflake: The ID of the emoji (or null if a standard Unicode emoji)
    pub name: Option<String>, // The name of the emoji
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Component {
    #[serde(rename = "type")]
    pub component_type: i32, // Component type
    pub label: Option<String>, // Text that appears on the button (if type is Button)
    pub style: Option<i32>,    // A button style (if type is Button)
    pub custom_id: Option<String>, // A developer-defined identifier for the component (if type is Button, SelectMenu)
    pub url: Option<String>,       // A URL for link-style buttons
    pub disabled: Option<bool>,    // Whether the component is disabled
    pub components: Option<Vec<Component>>, // A list of child components
    pub emoji: Option<ReactionEmoji>, // The emoji to display on the component
    pub min_values: Option<i32>,   // The minimum number of values that must be chosen
    pub max_values: Option<i32>,   // The maximum number of values that can be chosen
    pub options: Option<Vec<SelectOption>>, // A list of select menu options
    pub placeholder: Option<String>, // A placeholder for the component
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SelectOption {
    pub label: String,                // The user-facing name of the option
    pub value: String,                // The developer-defined value of the option
    pub description: Option<String>,  // An additional description of the option
    pub emoji: Option<ReactionEmoji>, // The emoji to display on the option
    pub default: Option<bool>,        // Whether this option is selected by default
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StickerItem {
    pub id: String,       // Snowflake: ID of the sticker
    pub name: String,     // Name of the sticker
    pub format_type: i32, // Type of sticker format
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Attachment {
    pub id: String,                   // Snowflake: Attachment ID
    pub filename: String,             // Name of file attached
    pub description: Option<String>,  // Description for the file (max 1024 characters)
    pub content_type: Option<String>, // Media type of file
    pub size: i32,                    // Size of file in bytes
    pub url: String,                  // Source URL of file
    pub proxy_url: String,            // A proxied URL of file
    pub height: Option<i32>,          // Height of file (if image)
    pub width: Option<i32>,           // Width of file (if image)
    pub ephemeral: Option<bool>,      // Whether this attachment is ephemeral
}
