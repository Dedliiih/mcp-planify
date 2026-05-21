use std::path::{PathBuf, Path};
use dirs::home_dir;
use crate::error::PlanifyError;

pub fn resolve_db_path() -> Result<PathBuf, PlanifyError> {
    let mut tried_paths: Vec<PathBuf> = vec![];

    if let Ok(path) = std::env::var("PLANIFY_DB_PATH") {
        let p = PathBuf::from(&path);
        if p.exists() { return Ok(p); }
        tried_paths.push(p);
    }

    if let Some(home) = home_dir() {
        let flatpak = home.join(".var/app/io.github.alainm23.planify/data/io.github.alainm23.planify/database.db");
        if flatpak.exists() { return Ok(flatpak); }
        tried_paths.push(flatpak);
    }

    Err(PlanifyError::DbNotFound {
        searched: tried_paths, hint: "Please set the PLANIFY_DB_PATH environment variable to a valid path or verify your Planify installation.".to_string() 
    })
}

use std::sync::{Arc, Mutex};
use rusqlite::Connection;

pub struct DbPool {
    pool: Arc<Mutex<Connection>>,
}

#[derive(Debug)]
#[expect(dead_code)]  
pub struct Project {
    pub id: String,
    pub name: String,
}

impl DbPool {
    pub fn new(path: &Path) -> Result<Self, PlanifyError> {
        let conn = Connection::open(path)?;
        conn.execute_batch("PRAGMA journal_mode=WAL; PRAGMA busy_timeout=5000;")?;
        Ok(Self { pool: Arc::new(Mutex::new(conn)) })   
    }

    pub fn exec<F, T> (&self, f: F) -> Result<T, PlanifyError>
    where 
        F: FnOnce(&Connection) -> Result<T, PlanifyError>,
    {
        let guard = self.pool.lock().map_err(|err| {
            PlanifyError::DbLocked { 
                hint: format!("Thread error: {}", err)
            }
        })?;
        f(&guard)
    }

    pub fn list_projects(&self) -> Result<Vec<Project>, PlanifyError> {
        self.exec(|conn| {
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolve_path_when_flatpak_exists() {
        let tmp = std::env::temp_dir().join("test_planify");
        let db_path = tmp.join(".var/app/io.github.alainm23.planify/data/io.github.alainm23.planify/database.db");
        std::fs::create_dir_all(db_path.parent().unwrap()).unwrap();
        std::fs::File::create(&db_path).unwrap();

        unsafe { std::env::set_var("HOME", &tmp) };
        
        let result = resolve_db_path();

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), db_path);

        std::fs::remove_dir_all(&tmp).unwrap();
    }

    #[test]
    fn error_when_flatpak_does_not_exists() {
        let tmp = std::env::temp_dir().join("test_planify_empty");
        std::fs::create_dir_all(&tmp).unwrap();
        unsafe { std::env::set_var("HOME", &tmp) };

        let result = resolve_db_path();

        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), PlanifyError::DbNotFound { .. }));

        std::fs::remove_dir_all(&tmp).unwrap();
    }

    #[test]
    fn env_var_override() {
        let tmp = std::env::temp_dir().join("test_planify_env");
        let custom_db = tmp.join("custom.db");
        std::fs::create_dir_all(&tmp).unwrap();
        std::fs::File::create(&custom_db).unwrap();

        unsafe { std::env::set_var("PLANIFY_DB_PATH", &custom_db) };

        let result = resolve_db_path();

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), custom_db);

        unsafe { std::env::remove_var("PLANIFY_DB_PATH") };
        std::fs::remove_dir_all(&tmp).unwrap();
    }

    #[test]
    fn create_memory_pool() {
        let pool = DbPool::new(Path::new(":memory:"));
        assert!(pool.is_ok());
    }

    #[test]
    fn create_empty_projects_table() {
        let pool = DbPool::new(Path::new(":memory:")).unwrap();

        pool.exec(|conn| {
            conn.execute_batch(
                "CREATE TABLE Projects (id TEXT PRIMARY KEY, name TEXT NOT NULL);"
            ).unwrap();
            Ok(())
        }).unwrap();
        let projects = pool.list_projects().unwrap();
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
        let projects = pool.list_projects().unwrap();
        assert_eq!(projects.len(), 2);
        assert_eq!(projects[0].name, "Inbox");
    }
}