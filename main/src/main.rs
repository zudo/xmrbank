use clap::Parser;
use color_eyre::Result;
use main::args::Args;
use sqlx::migrate::MigrateDatabase;
use sqlx::mysql::MySqlPoolOptions;
use sqlx::MySql;
use sqlx::Pool;
use tracing_subscriber;
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::fmt;
use tracing_subscriber::prelude::*;
use tracing_subscriber::EnvFilter;
#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    dotenv::dotenv().ok(); // use ok here instead of unwrap because otherwise we can't pass .env file with docker-compose
    tracing_subscriber::registry()
        .with(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .from_env_lossy()
                .add_directive("tarpc=warn".parse()?),
        )
        .with(
            fmt::layer()
                .with_target(false)
                .with_file(true)
                .with_line_number(true),
        )
        .init();
    let args = Args::parse();
    let db_pool = setup_db_pool(&args).await?;
    Ok(())
}
async fn setup_db_pool(args: &Args) -> Result<Pool<MySql>> {
    if !sqlx::MySql::database_exists(&args.database_url)
        .await
        .unwrap_or(false)
    {
        sqlx::MySql::create_database(&args.database_url).await?;
        tracing::info!("Database created");
    } else {
        tracing::info!("Database already exists");
    }
    let db_pool = MySqlPoolOptions::new()
        .max_connections(50)
        .connect(&args.database_url)
        .await?;
    sqlx::migrate!("../migrations").run(&db_pool).await?;
    tracing::info!("Migrations applied");
    Ok(db_pool)
}
