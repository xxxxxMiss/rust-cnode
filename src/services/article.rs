use crate::{
    error::Error,
    types::{article::ArticleListItem, ApiResponse},
};

use super::request_get;

pub async fn get_articles() -> Result<ApiResponse<Vec<ArticleListItem>>, Error> {
    request_get::<ApiResponse<Vec<ArticleListItem>>>("/topics".to_string()).await
}
