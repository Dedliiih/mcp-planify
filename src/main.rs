use crate::database::connection::DbPool;
use crate::server::PlanifyServer;
use tokio;
use rmcp::ServiceExt;
use rmcp::transport::io;

mod server;
mod database;
mod error;

#[tokio::main]
async fn main() {
    let db_path = database::connection::resolve_db_path().unwrap_or_else(|err| {
        eprintln!("Error: {:?}", err);
        std::process::exit(1);
    });
    eprintln!("Resolved database path: {}", db_path.display());

    let pool = DbPool::new(&db_path).unwrap();
    
    let server = PlanifyServer { pool };
    server.serve(io::stdio()).await.unwrap().waiting().await.unwrap();
}