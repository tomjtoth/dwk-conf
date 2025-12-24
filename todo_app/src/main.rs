use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};

use dioxus::prelude::*;
use once_cell::sync::Lazy;
use reqwest::header::LOCATION;

fn main() {
    dioxus::launch(App);
}

struct ServerState {
    src: String,
    timestamp: SystemTime,
}

static SERVER_STATE: Lazy<Mutex<ServerState>> = Lazy::new(|| {
    Mutex::new(ServerState {
        src: String::new(),
        timestamp: UNIX_EPOCH,
    })
});

// #[cfg(feature = "server")]
fn get_pic() -> impl Future<Output = Result<reqwest::Response, reqwest::Error>> {
    reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .unwrap()
        .get("https://picsum.photos/1200")
        .send()
}

// #[cfg(feature = "server")]
async fn get_src() -> Option<String> {
    if let Ok(mut state) = SERVER_STATE.lock() {
        let now = SystemTime::now();

        if now.duration_since(state.timestamp).unwrap().as_secs() > 10 * 1 {
            if let Ok(res) = get_pic().await {
                if res.status().is_redirection() {
                    if let Some(location) = res.headers().get(LOCATION) {
                        if let Ok(url) = location.to_str() {
                            let src = String::from(url);
                            state.src = src.clone();
                            state.timestamp = now;
                            return Some(src);
                        }
                    }
                }
            }
        }
    }

    None
}

#[component]
pub fn App() -> Element {
    let mut value = use_signal(|| String::new());

    let src = use_server_future(|| async { get_src().await }).unwrap();

    rsx! {
        h1 { "The project App" }
        img { src }
        form { onsubmit: |evt| evt.prevent_default(),
            input {
                value,
                max: 140,
                onchange: move |evt| value.set(evt.value()),
            }
            button { "Create todo" }
        }
        ul {
            li { "todo1" }
            li { "todo2" }
            li { "todo3" }
        }
    }
}

// #[tokio::main]
// async fn main() {
//     let app = Router::new().route("/", get(|| async { "Hello, World!" }));

//     let port = env::var("PORT").unwrap_or(String::from("3000"));

//     let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", &port))
//         .await
//         .unwrap();

//     println!("Server started at port {}", &port);
//     axum::serve(listener, app).await.unwrap();
// }
