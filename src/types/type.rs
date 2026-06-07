use crate::database::items::Item;
use crate::database::projects::Project;
use schemars::JsonSchema;
use serde::Serialize;

#[derive(Serialize, JsonSchema)]
pub struct ProjectList {
    pub projects: Vec<Project>,
}

#[derive(Serialize, JsonSchema)]
pub struct ItemList {
    pub items: Vec<Item>,
}
