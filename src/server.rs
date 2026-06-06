use crate::database::connection::DbPool;
use crate::database::projects::Project;
use crate::database::{items, projects};
use crate::parameters::item_params::{ListItemsParameters, CreateItemParams, CompleteItemParams, DeleteItemParams, UpdateItemParams};
use crate::parameters::project_params::{CreateProjectParams, UpdateProjectParams};
use crate::types::types::{ItemList, ProjectList};
use rmcp::handler::server::wrapper::{Json, Parameters};
use rmcp::{ErrorData, ServerHandler, tool, tool_handler, tool_router};

#[allow(dead_code)]
pub struct PlanifyServer {
    pub pool: DbPool,
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

    #[tool(name = "update_project", description = "Update a project by ID")]
    fn update_project(
        &self,
        params: Parameters<UpdateProjectParams>,
    ) -> Result<Json<Project>, ErrorData> {
        let request = params.0;
        projects::update_project(
            &self.pool,
            &request.project_id,
            request.name.as_deref(),
            request.description.as_deref(),
        )
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
        params: Parameters<CreateItemParams>,
    ) -> Result<Json<items::Item>, ErrorData> {
        let request = params.0;

        items::create_item(
            &self.pool,
            &request.content,
            &request.project_id,
            &request.description,
            request.priority,
            request.due.as_deref(),
            request.labels.as_deref(),
            request.parent_id.as_deref(),
        )
        .map(Json)
        .map_err(|e| ErrorData::internal_error(e.to_string(), None))
    }

    #[tool(name = "complete_item", description = "Mark a task as completed")]
    fn complete_item(
        &self,
        params: Parameters<CompleteItemParams>
    ) -> Result<Json<items::Item>, ErrorData> {
        let request = params.0;

        items::complete_item(&self.pool, &request.item_id)
            .map(Json)
            .map_err(|e| ErrorData::internal_error(e.to_string(), None))
    }

    #[tool(name = "delete_item", description = "Soft-delete a task by ID")]
    fn delete_item(
        &self,
        params: Parameters<DeleteItemParams>,
    ) -> Result<(), ErrorData> {
        let request = params.0;

        items::delete_item(&self.pool, &request.item_id)
            .map_err(|e| ErrorData::internal_error(e.to_string(), None))
    }

    #[tool(name = "update_item", description = "Update a created item by ID")]
    fn update_item(
        &self,
        params: Parameters<UpdateItemParams>,
    ) -> Result<Json<items::Item>, ErrorData> {
        let request = params.0;

        items::update_item(
            &self.pool, 
            &request.item_id, 
            request.content.as_deref(), 
            &request.description, 
            request.priority, 
            request.due.as_deref(), 
            request.labels.as_deref())
        .map(Json)
        .map_err(|e| ErrorData::internal_error(e.to_string(), None))
    }
}

#[tool_handler]
impl ServerHandler for PlanifyServer {}
