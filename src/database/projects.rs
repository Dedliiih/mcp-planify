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
        let mut stmt = conn.prepare("SELECT id, name, description, color FROM Projects")?;

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

#[warn(dead_code)]
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


#[cfg(test)]
mod tests {
    use super::*;
    #[allow(unused_imports)]
    use std::path::Path;

    #[test]
    fn create_empty_projects_table() {
        let pool = DbPool::new(Path::new(":memory:")).unwrap();

        pool.exec(|conn| {
            conn.execute_batch("CREATE TABLE Projects (id TEXT PRIMARY KEY, name TEXT NOT NULL);")
                .unwrap();
            Ok(())
        })
        .unwrap();
        let projects = list_projects(&pool).unwrap();
        assert!(projects.is_empty());
    }

    #[test]
    fn list_projects_returns_data() {
        let pool = DbPool::new(Path::new(":memory:")).unwrap();
        pool.exec(|conn| {
            conn.execute_batch(
                "CREATE TABLE Projects (id TEXT PRIMARY KEY, name TEXT NOT NULL);
                    INSERT INTO Projects VALUES ('1', 'Inbox');
                    INSERT INTO Projects VALUES ('2', 'Personal');",
            )
            .unwrap();
            Ok(())
        })
        .unwrap();
        let projects = list_projects(&pool).unwrap();
        assert_eq!(projects.len(), 2);
        assert_eq!(projects[0].name, "Inbox");
    }
}
