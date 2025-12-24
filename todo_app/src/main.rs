use dioxus::prelude::*;

#[cfg(feature = "server")]
mod server;

fn main() {
    #[cfg(feature = "server")]
    {
        server::init_server_state();

        let ip = std::env::var("IP").unwrap_or(String::from("127.0.0.1"));
        let port = std::env::var("PORT").unwrap_or(String::from("3000"));
        println!("TODO app listening at {}:{}/", ip, port);
    }

    dioxus::launch(App);
}

#[get("/img")]
async fn get_img_src() -> Result<String> {
    let res = server::refresh_state_if_needed().await.unwrap();
    dioxus::Ok(res)
}

#[component]
pub fn App() -> Element {
    let src = use_server_future(|| async {
        if let Ok(res) = get_img_src().await {
            return Some(res);
        }

        None
    })
    .expect("use_server_future went wrong");

    rsx! {
        document::Stylesheet { href: asset!("/assets/tailwind.css") }
        h1 { "The project App" }
        img { src, class: "max-w-100" }
        h3 { "DevOps with Kubernetes 2025" }
    }
}
