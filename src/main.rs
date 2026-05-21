use crate::database::connection::DbPool;

mod database;
mod error;

fn main() {
    let db_path = database::connection::resolve_db_path().unwrap_or_else(|err| {
        eprintln!("Error: {:?}", err);
        std::process::exit(1);
    });
    println!("Resolved database path: {}", db_path.display());

    let pool = DbPool::new(&db_path).unwrap();
    
    let projects = pool.list_projects();

    println!("{:?}", projects);
}