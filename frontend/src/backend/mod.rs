use gloo_net::http::Request;
use serde::Serialize;
use web_sys::{RequestCredentials, window};
use yew::platform::spawn_local;

#[derive(Serialize)]
struct ContentViewRequest {
    content_id: String,
}

pub fn register_content_view(content_id: &str) {
    let payload = ContentViewRequest {
        content_id: content_id.into(),
    };

    let host = if let Some(host) = get_host() {
        host
    } else {
        return;
    };

    let endpoint = format!("http://{}/api/v1/portfolio/track/content-view", host);

    spawn_local(async move {
        let _ = Request::post(&endpoint)
            .header("Content-Type", "application/json")
            .credentials(RequestCredentials::Include)
            .body(serde_json::to_string(&payload).unwrap())
            .expect("Failed to create request")
            .send()
            .await;
    });
}

pub fn get_host() -> Option<String> {
    if let Some(win) = window() {
        if let Ok(mut host) = win.location().host() {
            #[cfg(debug_assertions)]
            {
                host = host.replace(":8000", ":8080");
            }
            Some(host)
        } else {
            #[cfg(debug_assertions)]
            web_sys::console::log_1(&"no host".into());
            None
        }
    } else {
        #[cfg(debug_assertions)]
        web_sys::console::log_1(&"no active window".into());
        None
    }
}

#[allow(unused)]
pub fn get_frontnend_root_endpoint() -> String {
    let host = get_host().unwrap_or_else(|| {
        "bcastellano.com".to_string()
    });
    format!("http://{}/portfolio", host)
}
