use gloo_net::http::Request;
use serde::{Serialize};
use yew::platform::spawn_local;

#[derive(Serialize)]
struct ContentViewRequest {
    content_id: String,
}

pub fn register_content_view(content_id: &str) {
    let payload = ContentViewRequest {
        content_id: content_id.into(),
    };

    let endpoint = "http://localhost:8080/api/v1/portfolio/track/content-view";
    
    spawn_local(async move {
        let _ = Request::post(endpoint)
            .header("Content-Type", "application/json")
            .body(serde_json::to_string(&payload).unwrap())
            .expect("Failed to create request")
            .send()
            .await;
    });
}
