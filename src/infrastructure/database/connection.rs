
use dotenv::dotenv;
use std::env;
use std::sync::Arc;
use deadpool_postgres::Runtime::Tokio1;
use tokio_postgres::NoTls;

pub type Pool = deadpool_postgres::Pool;
pub async fn establish_connection() -> Arc<Pool> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let config = deadpool_postgres::Config {
        host: Some(database_url),
        user: env::var("DB_USER").ok(),
        password: env::var("DB_PASSWORD").ok(),
        dbname: env::var("DB_NAME").ok(),
        ..Default::default()
    };

    let pool = config.create_pool(Option::from(Tokio1), NoTls).expect("Failed to create pool.");
    Arc::new(pool)
}