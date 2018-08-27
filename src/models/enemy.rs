use crate::prelude::Either;

use super::{Metadata, GamePatch};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Enemy {
  #[serde(flatten)]
  pub metadata: Metadata,
  pub game_content_links: Either<[!; 0], serde_json::Value>,
  pub game_patch: Option<GamePatch>,
  #[serde(with = "crate::routes::int_bool")]
  pub starts_with_vowel: bool,
}
