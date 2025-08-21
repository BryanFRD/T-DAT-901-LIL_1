use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use super::source::Source;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct News {
    pub title: String,
    pub description: String,
    pub source: Source,
    pub url: String,
    pub image_url: String,
    pub published_at: Option<NaiveDateTime>,
}