use std::{
    env,
    fs::File,
    io::{SeekFrom, prelude::*},
    sync::Arc,
};

use axum::{Router, routing::get};

#[tokio::main]
async fn main() {
    let log_path = env::var("LOG_PATH").unwrap_or(String::from("logs/output.log"));

    let mut file = Arc::new(File::options().read(true).open(log_path).unwrap());

    let app = Router::new().route(
        "/",
        get(|| async move {
            let mut contents = String::new();

            let _ = file.seek(SeekFrom::Start(0));
            let _ = file.read_to_string(&mut contents);

            contents
        }),
    );

    let port = env::var("PORT").unwrap_or(String::from("3000"));

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", &port))
        .await
        .unwrap();

    println!("Server started at port {}", &port);
    axum::serve(listener, app).await.unwrap();
}
