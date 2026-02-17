pub mod collection;
pub mod matcher;
pub mod scorer;

pub use collection::RecipeCollection;
pub use matcher::{find_recipes_by_name, MatchResult, DEFAULT_THRESHOLD};
pub use scorer::{score_label, score_recipe, DimensionScore, RecipeScore};
