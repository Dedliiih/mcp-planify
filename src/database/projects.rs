use crate::database::connection::DbPool;
use crate::error::PlanifyError;
use rusqlite::params;
use schemars::JsonSchema;
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize, JsonSchema)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
}

pub fn list_projects(pool: &DbPool) -> Result<Vec<Project>, PlanifyError> {
    pool.exec(|conn| {
        let mut stmt = conn.prepare("SELECT id, name, description, color FROM Projects WHERE is_deleted = 0")?;

        let projects = stmt
            .query_map((), |row| {
                Ok(Project {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    description: row.get(2)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(projects)
    })
}

pub fn create_project(
    pool: &DbPool, 
    name: &str,
    description: &Option<String>,
) -> Result<Project, PlanifyError> {
    let id = Uuid::new_v4().to_string();

    let stmt = String::from(
        "
        INSERT INTO Projects (
            id, name, color, backend_type, inbox_project, team_inbox,
            child_order, is_deleted, is_archived, is_favorite, shared,
            view_style, sort_order, parent_id, collapsed, icon_style, emoji,
            show_completed, description, due_date, inbox_section_hidded,
            sync_id, source_id, calendar_url, sorted_by
        ) VALUES (
            ?1, ?2, 'blue', 'none', 0, 0,
            1000, 0, 0, 0, 0,
            'list', 'asc', '', 0, 'progress', '',
            1, COALESCE(?3, ''), '', 0,
            '', 'local', '', 'manual'
        )
        "
    );

    pool.exec(|conn| {
        conn.execute(
            &stmt, 
            params![id, name, description]
        )?;

        Ok(conn.query_row(
            "SELECT id, name, description FROM Projects WHERE id = ?1",
            params![&id],
            |row| {
                Ok(Project {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    description: row.get(2)?
                })
            }
        )?)
    })
}

pub fn update_project(
    pool: &DbPool,
    project_id: &str,
    name: Option<&str>,
    description: Option<&str>,
) -> Result<Project, PlanifyError> {
    let stmt = String::from(
        "UPDATE Projects
         SET name = COALESCE(?1, name),
             description = COALESCE(?2, description)
         WHERE id = ?3" 
    );

    pool.exec(|conn| {
        conn.execute(&stmt, params![name, description, project_id])?;

        Ok(conn.query_row(
            "SELECT id, name, description FROM Projects WHERE id = ?1",
            params![project_id],
            |row| Ok(Project {
                id: row.get(0)?,
                name: row.get(1)?,
                description: row.get(2)?,
            }),
        )?)
    })
}

pub fn delete_project(pool: &DbPool, project_id: &str) -> Result<(), PlanifyError> {
    pool.exec(|conn| {
        conn.execute(
            "UPDATE Items SET is_deleted = 1 WHERE project_id = ?1",
            params![project_id],
        )?;
        conn.execute(
            "UPDATE Projects SET is_deleted = 1 WHERE id = ?1",
            params![project_id],
        )?;
        Ok(())
    })
}

