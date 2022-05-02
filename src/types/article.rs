use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Author {
    pub avatar_url: String,
    pub loginname: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ArticleListItem {
    pub author: Author,
    pub title: String,
    pub author_id: String,
    pub content: String,
    pub create_at: String,
    pub good: bool,
    pub id: String,
    pub last_reply_at: String,
    pub reply_count: u32,
    pub tab: String,
    pub top: bool,
    pub visit_count: u32,
}
