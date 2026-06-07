use serde::{Serialize};
use schemars::JsonSchema;
use crate::database::projects::Project;
use crate::database::items::Item;

#[derive(Serialize, JsonSchema)]
pub struct ProjectList {
    pub projects: Vec<Project>,
}

#[derive(Serialize, JsonSchema)]
pub struct ItemList {
    pub items: Vec<Item>,
}
