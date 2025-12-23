use std::{
    env,
    fs::File,
    io::{Seek, SeekFrom, Write},
    sync::{
        Arc, Mutex,
        atomic::{AtomicU8, Ordering},
    },
};

use axum::{Router, extract::State, routing::get};

struct AppState {
    counter: AtomicU8,
    file: Arc<Mutex<File>>,
}

#[tokio::main]
async fn main() {
    let pong_path = env::var("PONG_PATH").unwrap_or(String::from("data/pong"));

    let state = Arc::new(AppState {
        counter: AtomicU8::new(0),
        file: Arc::new(Mutex::new(
            File::options()
                .write(true)
                .create(true)
                .open(&pong_path)
                .expect(&format!(r#"unable to open PONG_PATH="{}""#, pong_path)),
        )),
    });

    let app = Router::new()
        .route(
            "/pingpong",
            get(|State(state): State<Arc<AppState>>| async move {
                let current = state.counter.fetch_add(1, Ordering::SeqCst);

                if let Ok(mut file) = state.file.lock() {
                    let _ = file.set_len(0);
                    let _ = file.seek(SeekFrom::Start(0));
                    let _ = write!(file, "{}", current);
                }

                format!("pong {}", current)
            }),
        )
        .with_state(state);

    let port = env::var("PORT").unwrap_or(String::from("3000"));

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", &port))
        .await
        .unwrap();

    println!("listening at :{}/pingpong", &port);
    axum::serve(listener, app).await.unwrap();
}
