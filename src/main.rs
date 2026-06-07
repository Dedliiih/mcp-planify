use std::process;

use crate::database::connection::DbPool;
use crate::server::PlanifyServer;
use rmcp::ServiceExt;
use rmcp::transport::io;

mod database;
mod error;
mod parameters;
mod server;
mod types;

#[tokio::main]
async fn main() {
    let db_path = database::connection::resolve_db_path().unwrap_or_else(|err| {
        eprintln!("Error: {:?}", err);
        std::process::exit(1);
    });
    eprintln!("Resolved database path: {}", db_path.display());

    let pool = DbPool::new(&db_path)
        .unwrap_or_else(|_| panic!("Failed to open database at: {:?}", db_path));

    let server = PlanifyServer { pool };

    match server.serve(io::stdio()).await {
        Ok(s) => {
            if let Err(e) = s.waiting().await {
                eprintln!("MCP server exited with error : {e}");
            }
        }

        Err(e) => {
            eprintln!("Failed to start MCP server: {e}");
            process::exit(1);
        }
    }
}
