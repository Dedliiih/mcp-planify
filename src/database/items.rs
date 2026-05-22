use crate::{database::connection::DbPool, error::PlanifyError};
use uuid::Uuid;
use rusqlite::params;
use rusqlite::types::ToSql;
use serde::Serialize;
use schemars::JsonSchema;

#[derive(Debug, Serialize, JsonSchema)]
#[allow(dead_code)]
pub struct Item {
    pub id: String,
    pub content: String,
    pub description: Option<String>,
    pub priority: i64,
    pub due: Option<String>,
    pub labels: Option<String>,
    pub project_id: String,
    pub checked: bool,
    pub added_at: String,
}

#[allow(dead_code)]
pub fn create_item(pool: &DbPool, content: &str, project_id: &str, description: &Option<String>, 
    priority: Option<i64>, due: Option<&str>, labels: Option<&str>)  -> Result<Item, PlanifyError> {
    let id = Uuid::new_v4().to_string();
    let now = chrono::Local::now().format("%Y-%m-%dT%H:%M:%S%z").to_string();
    let priority = priority.unwrap_or(1);
    let due = due.unwrap_or(r#"{"date":"","timezone":"","is_recurring":false,"recurrency_type":"6","recurrency_interval":"0","recurrency_weeks":"","recurrency_count":"0","recurrency_end":""}"#);
    let labels = labels.unwrap_or("");

    pool.exec(|conn| {
        conn.execute(
            "INSERT INTO Items (id, content, description, due, added_at, project_id,
                                priority, labels, checked, child_order, is_deleted,
                                day_order, collapsed, pinned, item_type, updated_at,
                                section_id)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, 0, 1000, 0,
                      0, 0, 0, 'task', ?9, '')",
            params![id, content, description, due, now, project_id, priority, labels, now],
        )?;

        Ok(conn.query_row(
            "SELECT id, content, description, priority, due, labels,
                    project_id, checked, added_at
             FROM Items WHERE id = ?1",
            [&id],
            |row| Ok(Item {
                id: row.get(0)?,
                content: row.get(1)?,
                description: row.get(2)?,
                priority: row.get(3)?,
                due: row.get(4)?,
                labels: row.get(5)?,
                project_id: row.get(6)?,
                checked: row.get(7)?,
                added_at: row.get(8)?,
            }),
        )?)
    })
}

#[allow(dead_code)]
pub fn delete_item(pool: &DbPool, item_id: &str) -> Result<(), PlanifyError> {
    pool.exec(|conn| {
        conn.execute(
            "UPDATE Items SET is_deleted = 1 WHERE id = ?1",
            params![item_id],
        )?;
        Ok(())
    })
}

#[allow(dead_code)]
pub fn complete_item(pool: &DbPool, item_id: &str) -> Result<Item, PlanifyError> {
    let now = chrono::Local::now().format("%Y-%m-%dT%H:%M:%S%z").to_string();

    pool.exec(|conn| {
        conn.execute(
            "UPDATE Items SET checked = 1, completed_at = ?1 WHERE id = ?2",
            params![now, item_id],
        )?;
        Ok(conn.query_row(
            "SELECT id, content, description, priority, due, labels,
                    project_id, checked, added_at
            FROM Items WHERE id = ?1",
            [item_id],
            |row| Ok(Item {
                id: row.get(0)?,
                content: row.get(1)?,
                description: row.get(2)?,
                priority: row.get(3)?,
                due: row.get(4)?,
                labels: row.get(5)?,
                project_id: row.get(6)?,
                checked: row.get(7)?,
                added_at: row.get(8)?,
            }),
        )?)
    })
}

#[allow(dead_code)]
pub fn list_items(
    pool: &DbPool,
    project_id: Option<&str>,
    completed: Option<bool>,
    priority: Option<i64>,
) -> Result<Vec<Item>, PlanifyError> {
    pool.exec(|conn| {
        let mut sql = String::from(
            "SELECT id, content, description, priority, due, labels,
                    project_id, checked, added_at
             FROM Items WHERE is_deleted = 0"
        );
        let mut params: Vec<Box<dyn ToSql>> = vec![];

        if let Some(pid) = project_id {
            sql.push_str(" AND project_id = ?");
            params.push(Box::new(pid.to_string()));
        }
        if let Some(c) = completed {
            sql.push_str(" AND checked = ?");
            params.push(Box::new(if c { 1 } else { 0 }));
        }
        if let Some(p) = priority {
            sql.push_str(" AND priority = ?");
            params.push(Box::new(p));
        }
        sql.push_str(" ORDER BY priority DESC, added_at ASC");
        let mut stmt = conn.prepare(&sql)?;
        let param_refs: Vec<&dyn rusqlite::types::ToSql> = params.iter().map(|p| p.as_ref()).collect();
        let items = stmt.query_map(param_refs.as_slice(), |row| {
            Ok(Item {
                id: row.get(0)?,
                content: row.get(1)?,
                description: row.get(2)?,
                priority: row.get(3)?,
                due: row.get(4)?,
                labels: row.get(5)?,
                project_id: row.get(6)?,
                checked: row.get(7)?,
                added_at: row.get(8)?,
            })
        })?.collect::<Result<Vec<_>, _>>()?;
        Ok(items)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    // Setup table helper
    fn setup_items_table(pool: &DbPool) {
        pool.exec(|conn| {
            conn.execute_batch(
                "CREATE TABLE IF NOT EXISTS Items (
                    id TEXT PRIMARY KEY, content TEXT NOT NULL,
                    description TEXT, due TEXT, added_at TEXT,
                    completed_at TEXT, updated_at TEXT,
                    section_id TEXT, project_id TEXT, parent_id TEXT,
                    priority INTEGER DEFAULT 1, child_order INTEGER DEFAULT 0,
                    checked INTEGER DEFAULT 0, is_deleted INTEGER DEFAULT 0,
                    day_order INTEGER DEFAULT 0, collapsed INTEGER DEFAULT 0,
                    pinned INTEGER DEFAULT 0, labels TEXT
                );"
            ).unwrap();
            Ok(())
        }).unwrap();
    }
   
    #[test]
    fn create_item_with_only_required_fields_test() {
        let pool = DbPool::new(Path::new(":memory")).unwrap();
        setup_items_table(&pool);

        let item = create_item(
            &pool, "content", "proj-1", &None, None, None, None,
        ).unwrap();

        assert_eq!(item.content, "content");
        assert_eq!(item.project_id, "proj-1");
    }

    #[test]
    fn create_item_with_all_fields_test() {
         let pool = DbPool::new(Path::new(":memory:")).unwrap();
        setup_items_table(&pool);

        let item = create_item(
            &pool, "Tarea importante", "proj-42",
            &Some("Descripción detallada".to_string()),
            Some(4),
            Some(r#"{"date":"2026-06-15"}"#),
            Some("label-1,label-2"),
        ).unwrap();

        assert_eq!(item.content, "Tarea importante");
        assert_eq!(item.description.unwrap(), "Descripción detallada");
        assert_eq!(item.priority, 4);
        assert_eq!(item.due.unwrap(), r#"{"date":"2026-06-15"}"#);
        assert_eq!(item.labels.unwrap(), "label-1,label-2");
    }

     #[test]
    fn verify_new_item_persistence() {
        let pool = DbPool::new(Path::new(":memory:")).unwrap();
        setup_items_table(&pool);
        let create_item = create_item(
            &pool, "Persistente", "proj-1",
            &None, None, None, None,
        ).unwrap();

        let read_created_item = pool.exec(|conn| {
            Ok(conn.query_row(
                "SELECT content FROM Items WHERE id = ?1",
                [&create_item.id],
                |row| row.get::<_, String>(0),
            ))
        }).unwrap();

        assert_eq!(read_created_item.unwrap(), create_item.content);
    }

    #[test]
    fn check_item_test() {
        let pool = DbPool::new(Path::new(":memory:")).unwrap();
        setup_items_table(&pool);

        let item = create_item(&pool, "Para completar", "proj-1",
            &None, None, None, None).unwrap();

        assert!(!item.checked);
        let completado = complete_item(&pool, &item.id).unwrap();
        assert!(completado.checked);
    }

    #[test]
    fn complete_item_verifica_completed_at() {
        let pool = DbPool::new(Path::new(":memory:")).unwrap();
        setup_items_table(&pool);

        let item = create_item(&pool, "Con timestamp", "proj-1",
            &None, None, None, None).unwrap();

        let completado = complete_item(&pool, &item.id).unwrap();

        assert!(completado.checked);

        let completed_at: String = pool.exec(|conn| {
            Ok(conn.query_row(
                "SELECT completed_at FROM Items WHERE id = ?1",
                [&completado.id],
                |row| Ok(row.get::<_, String>(0)?),
            ).unwrap_or_default())
        }).unwrap();
        assert!(!completed_at.is_empty());
    }

    #[test]
    fn list_items_without_filter_returns_all_not_eliminated() {
        let pool = DbPool::new(Path::new(":memory:")).unwrap();
        setup_items_table(&pool);

        create_item(&pool, "Tarea A", "proj-1", &None, None, None, None).unwrap();
        create_item(&pool, "Tarea B", "proj-1", &None, None, None, None).unwrap();
        let items = list_items(&pool, None, None, None).unwrap();

        assert_eq!(items.len(), 2);
    }   

    #[test]
    fn list_items_filter_by_project_id() {
        let pool = DbPool::new(Path::new(":memory:")).unwrap();
        setup_items_table(&pool);

        create_item(&pool, "Proyecto 1", "proj-A", &None, None, None, None).unwrap();
        create_item(&pool, "Proyecto 2", "proj-B", &None, None, None, None).unwrap();
        let items = list_items(&pool, Some("proj-A"), None, None).unwrap();

        assert_eq!(items.len(), 1);
        assert_eq!(items[0].content, "Proyecto 1");
    }

    #[test]
    fn list_items_filter_by_completes() {
        let pool = DbPool::new(Path::new(":memory:")).unwrap();
        setup_items_table(&pool);
        create_item(&pool, "Pendiente", "proj-1", &None, None, None, None).unwrap();

        let item = create_item(&pool, "Completada", "proj-1", &None, None, None, None).unwrap();
        complete_item(&pool, &item.id).unwrap();

        let pendientes = list_items(&pool, None, Some(false), None).unwrap();

        assert_eq!(pendientes.len(), 1);
        assert_eq!(pendientes[0].content, "Pendiente");
    }

    #[test]
    fn delete_item_test() {
        let pool = DbPool::new(Path::new(":memory:")).unwrap();
        setup_items_table(&pool);
        let item = create_item(&pool, "Para borrar", "proj-1",
            &None, None, None, None).unwrap();
        delete_item(&pool, &item.id).unwrap();

        let items = list_items(&pool, None, None, None).unwrap();
        assert!(items.is_empty());
    }
}