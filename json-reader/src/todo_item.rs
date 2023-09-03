use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct TodoItem {
    #[serde(rename = "userId")]
    pub user_id: usize,
    pub id: usize,
    pub title: String,
    pub completed: bool,
}
