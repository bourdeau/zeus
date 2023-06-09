use rocket::serde::{Deserialize, Serialize};
use rocket_okapi::okapi::schemars::{self, JsonSchema};

use crate::models::post;

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct PaginatePosts {
    pub page: u64,
    pub posts_per_page: u64,
    pub num_pages: u64,
    pub posts: Vec<post::Model>,
}
