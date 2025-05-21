use dotenvy::dotenv;
use std::env;
use tokio_postgres::{NoTls, Error};

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().ok(); // Load .env
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let (client, connection) = tokio_postgres::connect(&db_url, NoTls).await?;

    // Spawn the connection task in the background
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    // Create table if not exists
    client
        .execute(
            "CREATE TABLE IF NOT EXISTS greetings (
                id SERIAL PRIMARY KEY,
                message TEXT NOT NULL
            )",
            &[],
        )
        .await?;

    // Insert a row
    client
        .execute("INSERT INTO greetings (message) VALUES ($1)", &[&"Hello, world!"])
        .await?;

    // Query the table
    let rows = client.query("SELECT id, message FROM greetings", &[]).await?;

    for row in rows {
        let id: i32 = row.get(0);
        let message: &str = row.get(1);
        println!("Greeting {}: {}", id, message);
    }

    Ok(())
}
