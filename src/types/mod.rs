use std::collections::HashMap;

use serde::{Deserialize, Serialize};

pub mod article;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ErrorInfo {
    pub errors: HashMap<String, Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ApiResponse<T> {
    pub data: T,
    pub success: bool,
}
