use std::path::{PathBuf, Path};
use dirs::home_dir;
use crate::error::PlanifyError;

pub fn resolve_db_path() -> Result<PathBuf, PlanifyError> {
    if let Ok(path) = std::env::var("PLANIFY_DB_PATH") {
        let p = PathBuf::from(&path);
        if p.exists() { return Ok(p); }
        return Err(PlanifyError::DbNotFound { 
            searched: vec![p], 
            hint: format!("PLANIFY_DB_PATH is set to {:?} but the file does not exist", path)
        })
    }

    if let Some(home) = home_dir() {
        let flatpak = home.join(".var/app/io.github.alainm23.planify/data/io.github.alainm23.planify/database.db");
        if flatpak.exists() { return Ok(flatpak); }
    }

    Err(PlanifyError::DbNotFound {
        searched: vec![], hint: "Please set the PLANIFY_DB_PATH environment variable to a valid path or verify your Planify installation.".to_string() 
    })
}

use std::sync::{Arc, Mutex};
use rusqlite::Connection;

pub struct DbPool {
    pool: Arc<Mutex<Connection>>,
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

  
}