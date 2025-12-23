use std::{
    env,
    fs::File,
    io::{SeekFrom, prelude::*},
    sync::Arc,
};

use axum::{Router, extract::State, routing::get};

#[derive(Clone)]
struct AppState {
    log_file: Arc<File>,
}

#[tokio::main]
async fn main() {
    let log_path = env::var("LOG_PATH").unwrap_or(String::from("logs/output.log"));

    let app_state = AppState {
        log_file: Arc::new(File::options().read(true).open(log_path).unwrap()),
    };

    let app = Router::new()
        .route(
            "/",
            get(
                |State(AppState { mut log_file }): State<AppState>| async move {
                    let mut contents = String::new();
                    let _ = log_file.seek(SeekFrom::Start(0));
                    let _ = log_file.read_to_string(&mut contents);

                    contents
                },
            ),
        )
        .with_state(app_state);

    let port = env::var("PORT").unwrap_or(String::from("3000"));

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", &port))
        .await
        .unwrap();

    println!("LOG_SERVER listening at :{}/", &port);
    axum::serve(listener, app).await.unwrap();
}
