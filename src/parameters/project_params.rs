use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Deserialize, JsonSchema, Default)]
#[allow(dead_code)]
pub struct CreateProjectParams {
    pub name: String,
    pub description: Option<String>
}

#[derive(Deserialize, JsonSchema, Default)]
#[allow(dead_code)]
pub struct UpdateProjectParams {
    pub project_id: String,
    pub name: Option<String>,
    pub description: Option<String>,
}

#[derive(Deserialize, JsonSchema, Default)]
#[allow(dead_code)]
pub struct DeleteProjectParams {
    pub project_id: String,
}