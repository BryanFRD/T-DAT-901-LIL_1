use std::fmt;
use colored::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Source {
  CoinTelegraph,
  CoinDesk,
}

impl fmt::Display for Source {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Source::CoinTelegraph => "COIN_TELEGRAPH",
            Source::CoinDesk => "COIN_DESK",
        };
        write!(f, "{}", name.green())
    }
}