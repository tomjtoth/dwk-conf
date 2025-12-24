use {
    dioxus::{fullstack::Lazy, prelude::info},
    reqwest::header::LOCATION,
    std::time::SystemTime,
    tokio::sync::Mutex,
};

struct ServerState {
    src: Option<String>,
    timestamp: SystemTime,
}

static SERVER_STATE: Lazy<Mutex<ServerState>> = Lazy::new(|| async {
    dioxus::Ok(Mutex::new(ServerState {
        src: get_src().await,
        timestamp: SystemTime::now(),
    }))
});

pub(super) fn init_server_state() {
    let _ = *SERVER_STATE;
}

async fn get_src() -> Option<String> {
    let res = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .unwrap()
        .get("https://picsum.photos/1200")
        .send()
        .await;

    if let Ok(res) = res {
        if res.status().is_redirection() {
            if let Some(header_value) = res.headers().get(LOCATION) {
                if let Ok(url) = header_value.to_str() {
                    return Some(url.to_string());
                }
            }
        }
    }
    None
}

pub(super) async fn refresh_state_if_needed() -> Option<String> {
    let now = SystemTime::now();
    let needs_update = {
        let st = SERVER_STATE.lock().await;
        now.duration_since(st.timestamp).unwrap().as_secs() > 5
    };

    if needs_update {
        tokio::spawn(async move {
            let url = get_src().await;

            let mut st = SERVER_STATE.lock().await;
            st.src = url;
            st.timestamp = SystemTime::now();
            info!("changed image src");
        });
    }

    SERVER_STATE.lock().await.src.clone()
}
