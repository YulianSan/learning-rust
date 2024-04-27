use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

pub async fn start_connection() -> Pool<Postgres> {
    let postgres_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&postgres_url)
        .await
        .expect("Failed to connect to postgres");

    match sqlx::migrate!("./src/databases/migrations")
        .run(&pool)
        .await
    {
        Ok(_) => {}
        Err(err) => {
            println!("Failed to run migrations: {}", err);
            std::process::exit(1);
        }
    };

    pool
}
