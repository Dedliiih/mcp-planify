use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Deserialize, JsonSchema)]
#[allow(dead_code)]
pub struct ListItemsParameters {
    pub project_id: Option<String>,
    pub completed: Option<bool>,
    pub priority: Option<i64>,
}

#[derive(Deserialize, JsonSchema, Default)]
#[allow(dead_code, clippy::too_many_arguments)]
pub struct CreateItemParams {
    pub content: String,
    pub project_id: String,
    pub description: Option<String>,
    pub priority: Option<i64>,
    pub due: Option<String>,
    pub labels: Option<String>,
    pub parent_id: Option<String>,
}

#[derive(Deserialize, JsonSchema, Default)]
#[allow(dead_code)]
pub struct CompleteItemParams {
    pub item_id: String,
}

#[derive(Deserialize, JsonSchema, Default)]
#[allow(dead_code)]
pub struct DeleteItemParams {
    pub item_id: String,
}

#[derive(Deserialize, JsonSchema, Default)]
#[allow(dead_code)]
pub struct UpdateItemParams {
    pub item_id: String,
    pub content: Option<String>,
    pub description: Option<String>,
    pub priority: Option<i64>,
    pub due: Option<String>,
    pub labels: Option<String>,
}
