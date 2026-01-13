use sqlx::postgres::PgPoolOptions;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::builder()
        // got a 403 without user_agent
        .user_agent("dummy_user_agent/0.1")
        .redirect(reqwest::redirect::Policy::none())
        .build()?;

    let resp = client
        .get("https://en.wikipedia.org/wiki/Special:Random")
        .send()
        .await?
        .error_for_status()?;

    let url = resp
        .headers()
        .get(reqwest::header::LOCATION)
        .expect("did not receive location header")
        .to_str()?;

    let todo_text = format!("Read {}", url);

    let database_url = env::var("DATABASE_URL").expect("missing env var DATABASE_URL");

    let pool = PgPoolOptions::new()
        .max_connections(1)
        .connect(&database_url)
        .await?;

    sqlx::query("INSERT INTO todos (todo) VALUES ($1)")
        .bind(&todo_text)
        .execute(&pool)
        .await?;

    println!("Inserted: {}", todo_text);

    Ok(())
}
