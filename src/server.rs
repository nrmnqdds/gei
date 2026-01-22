use anyhow::Result;
use std::sync::Arc;
use tonic::transport::Server;

mod service;

use gei::crypto;
use gei::db;
use service::schedule::schedule_indexer_server::ScheduleIndexerServer;
use service::ScheduleIndexerService;

#[tokio::main]
async fn main() -> Result<()> {
    // Configuration
    let addr = "0.0.0.0:50053".parse()?;
    let database_url =
        std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite://schedules.db".to_string());
    let encryption_key = std::env::var("ENCRYPTION_KEY").ok();

    println!("🚀 GEI Server Starting...");
    println!("📍 Address: {}", addr);
    println!("🗄️  Database: {}", database_url);

    // Initialize encryption key
    crypto::init_encryption_key(encryption_key.as_deref())
        .expect("Failed to initialize encryption key");
    println!("🔐 Encryption initialized");

    // Initialize database
    let db = db::Database::new(&database_url)
        .await
        .expect("Failed to connect to database");
    db.init_schema()
        .await
        .expect("Failed to initialize database schema");
    println!("✅ Database ready");

    let db = Arc::new(db);

    // Create the service
    let service = ScheduleIndexerService::new(db);

    println!("🎧 Server listening on {}", addr);
    println!();

    // Start the gRPC server
    Server::builder()
        .add_service(ScheduleIndexerServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}
