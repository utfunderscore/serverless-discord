use serde::{Deserialize, Serialize};
use crate::model::primitives::{Snowflake};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct User {
    id: Snowflake,
    username: String,
    discriminator: String,
    global_name: Option<String>,
    avatar: Option<String>,
    bot: Option<bool>,
    system: Option<bool>,
    mfa_enabled: Option<bool>,
    banner: Option<String>,
    accent_color: Option<i32>,
    locale: Option<String>,
    verified: Option<bool>,
    email: Option<String>,
    flags: Option<i32>,
    premium_type: Option<i32>,
    public_flags: Option<i32>,
    avatar_decoration_data: Option<AvatarDecorationData>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AvatarDecorationData {
    sku_id: Snowflake,
    assert: String,
}