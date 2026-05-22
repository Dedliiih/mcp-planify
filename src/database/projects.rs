use crate::database::connection::DbPool;
use crate::error::PlanifyError;

#[allow(unused_imports)]
use std::path::Path;

use serde::Serialize;
use schemars::JsonSchema;

#[derive(Debug, Serialize, JsonSchema)]
pub struct Project {
    pub id: String,
    pub name: String,
}

pub fn list_projects(pool: &DbPool) -> Result<Vec<Project>, PlanifyError> {
    pool.exec(|conn| {
        let mut stmt = conn.prepare("SELECT id, name FROM Projects")?;
        
        let projects = stmt.query_map((), |row| {
            Ok(Project {
                id: row.get(0)?,
                name: row.get(1)?
            })
        })?.collect::<Result<Vec<_>, _>>()?;

        Ok(projects)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_empty_projects_table() {
        let pool = DbPool::new(Path::new(":memory:")).unwrap();

        pool.exec(|conn| {
            conn.execute_batch(
                "CREATE TABLE Projects (id TEXT PRIMARY KEY, name TEXT NOT NULL);"
            ).unwrap();
            Ok(())
        }).unwrap();
        let projects = list_projects(&pool).unwrap();
        assert!(projects.is_empty());
    }

    #[test]
    fn list_projects_devuelve_datos() {
        let pool = DbPool::new(Path::new(":memory:")).unwrap();
        pool.exec(|conn| {
            conn.execute_batch(
                "CREATE TABLE Projects (id TEXT PRIMARY KEY, name TEXT NOT NULL);
                    INSERT INTO Projects VALUES ('1', 'Inbox');
                    INSERT INTO Projects VALUES ('2', 'Personal');"
            ).unwrap();
            Ok(())
        }).unwrap();
        let projects = list_projects(&pool).unwrap();
        assert_eq!(projects.len(), 2);
        assert_eq!(projects[0].name, "Inbox");
    }
}
