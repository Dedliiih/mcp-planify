use crate::database::connection::DbPool;
use crate::database::projects::Project;
use crate::database::{items, projects};
use rmcp::handler::server::wrapper::{Json, Parameters};
use rmcp::{ErrorData, ServerHandler, tool, tool_handler, tool_router};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, JsonSchema)]
struct ProjectList {
    projects: Vec<projects::Project>,
}

#[derive(Serialize, JsonSchema)]
struct ItemList {
    items: Vec<items::Item>,
}

#[allow(dead_code)]
pub struct PlanifyServer {
    pub pool: DbPool,
}

#[derive(Deserialize, JsonSchema)]
#[allow(dead_code)]
struct ListItemsParameters {
    project_id: Option<String>,
    completed: Option<bool>,
    priority: Option<i64>,
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
    item_id: String,
    content: Option<String>,
    description: Option<String>,
    priority: Option<i64>,
    due: Option<String>,
    labels: Option<String>
}

#[derive(Deserialize, JsonSchema, Default)]
#[allow(dead_code)]
pub struct CreateProjectParams {
    name: String,
    description: Option<String>
}

#[tool_router]
impl PlanifyServer {
    #[tool(name = "list_projects", description = "List all available projects")]
    fn list_projects(&self) -> Result<Json<ProjectList>, ErrorData> {
        projects::list_projects(&self.pool)
            .map(|projects| Json(ProjectList { projects }))
            .map_err(|e| ErrorData::internal_error(e.to_string(), None))
    }

    #[tool(name = "create_project", description = "Create a new project")]
    fn create_project(
        &self,
        params: Parameters<CreateProjectParams>
    ) -> Result<Json<Project>, ErrorData> {
        let request = params.0;
        projects::create_project(&self.pool, &request.name, &request.description)
        .map(Json)
        .map_err(|e| ErrorData::internal_error(e.to_string(), None))
    }

    #[tool(name = "list_items", description = "List items, optionally filtered by project, completion status or priority")]
    fn list_items(
        &self,
        params: Parameters<ListItemsParameters>,
    ) -> Result<Json<ItemList>, ErrorData> {
        let request = params.0;
        items::list_items(
            &self.pool,
            request.project_id.as_deref(),
            request.completed,
            request.priority,
        )
        .map(|items| Json(ItemList { items }))
        .map_err(|e| ErrorData::internal_error(e.to_string(), None))
    }

    #[tool(name = "create_item", description = "Create a new task")]
    fn create_item(
        &self,
        Parameters(CreateItemParams {
            content,
            project_id,
            description,
            priority,
            due,
            labels,
            parent_id,
        }): Parameters<CreateItemParams>,
    ) -> Result<Json<items::Item>, ErrorData> {
        items::create_item(
            &self.pool,
            &content,
            &project_id,
            &description,
            priority,
            due.as_deref(),
            labels.as_deref(),
            parent_id.as_deref(),
        )
        .map(Json)
        .map_err(|e| ErrorData::internal_error(e.to_string(), None))
    }

    #[tool(name = "complete_item", description = "Mark a task as completed")]
    fn complete_item(
        &self,
        Parameters(CompleteItemParams { item_id }): Parameters<CompleteItemParams>,
    ) -> Result<Json<items::Item>, ErrorData> {
        items::complete_item(&self.pool, &item_id)
            .map(Json)
            .map_err(|e| ErrorData::internal_error(e.to_string(), None))
    }

    #[tool(name = "delete_item", description = "Soft-delete a task by ID")]
    fn delete_item(
        &self,
        Parameters(DeleteItemParams { item_id }): Parameters<DeleteItemParams>,
    ) -> Result<(), ErrorData> {
        items::delete_item(&self.pool, &item_id)
            .map_err(|e| ErrorData::internal_error(e.to_string(), None))
    }

    #[tool(name = "update_item", description = "Update a created item by ID")]
    fn update_item(
        &self,
        Parameters(UpdateItemParams {
            item_id,
            content,
            description,
            priority,
            due,
            labels
        }) : Parameters<UpdateItemParams>,
    ) -> Result<Json<items::Item>, ErrorData> {
        items::update_item(&self.pool, &item_id, content.as_deref(), &description, priority, due.as_deref(), labels.as_deref())
        .map(Json)
        .map_err(|e| ErrorData::internal_error(e.to_string(), None))
    }
}

#[tool_handler]
impl ServerHandler for PlanifyServer {}
