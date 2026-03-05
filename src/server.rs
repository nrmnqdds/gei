use anyhow::Result;
use console::Style;
use std::sync::Arc;
use tonic::transport::Server;

mod service;

use dotenvy::dotenv;
use gei::crypto;
use gei::db;
use service::schedule::schedule_indexer_server::ScheduleIndexerServer;
use service::ScheduleIndexerService;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    // Configuration
    let addr = "0.0.0.0:50053".parse()?;
    let database_url =
        std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite://schedules.db".to_string());
    let encryption_key = std::env::var("ENCRYPTION_KEY").ok();

    // Initialize encryption key
    crypto::init_encryption_key(encryption_key.as_deref())
        .expect("Failed to initialize encryption key");

    // Initialize database
    let db = db::Database::new(&database_url)
        .await
        .expect("Failed to connect to database");
    db.init_schema()
        .await
        .expect("Failed to initialize database schema");

    let db = Arc::new(db);

    // Create the service
    let service = ScheduleIndexerService::new(db);

    print_intro();

    // Start the gRPC server
    Server::builder()
        .add_service(ScheduleIndexerServer::new(service))
        .serve(addr)
        .await?;
    // println!("🔐 Encryption initialized");
    // println!("✅ Database ready");

    Ok(())
}

pub fn print_intro() {
    // Using TrueColor (RGB) for a smooth gradient
    let red = Style::new().color256(196);
    let orange = Style::new().color256(208);
    let yellow = Style::new().color256(226);
    let green = Style::new().color256(46);
    let blue = Style::new().color256(21);
    let purple = Style::new().color256(129);

    println!("{}", red.apply_to(" ██████╗ ███████╗██╗"));

    println!("{}", orange.apply_to("██╔════╝ ██╔════╝██║"));

    println!("{}", yellow.apply_to("██║  ███╗█████╗  ██║"));

    println!("{}", green.apply_to("██║   ██║██╔══╝  ██║"));

    println!("{}", blue.apply_to("╚██████╔╝███████╗██║"));

    println!("{}", purple.apply_to(" ╚═════╝ ╚══════╝╚═╝"));

    println!("{}", yellow.apply_to("GoMaluum Entries Indexer"));

    println!(
        "{}",
        Style::new().yellow().apply_to(
            "===================================================================================="
        )
    );
    println!(
        "{}",
        Style::new()
            .blue()
            .apply_to("gRPC server listening on [::1]:50053")
    );
    println!(
        "{}",
        Style::new().yellow().apply_to(
            "===================================================================================="
        )
    );
}
