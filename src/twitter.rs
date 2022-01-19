use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct V2TwitterResponse<T> {
    pub data: T,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TwitterUser {
    pub id: String,
    pub name: String, // screen_name, @hoge
    pub username: String,
    pub profile_image_url: String,
}
