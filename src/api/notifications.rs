use axum::{Json, response, http};
use serde::{Deserialize};

#[derive(Deserialize, Debug)]
pub struct NotificationInput {
    summary: String,
    body: String,
    icon: Option<String>,
    app_name: Option<String>,
    timeout: Option<u32>,
}

pub async fn create(Json(payload): Json<NotificationInput>) -> impl response::IntoResponse {
    tracing::debug!("payload: {:?}", payload);

    let mut notification = notify_rust::Notification::new();
    notification.summary(&payload.summary).body(&payload.body);
    if let Some(ref icon) = payload.icon {
        notification.icon(icon);
    }
    if let Some(ref app_name) = payload.app_name {
        notification.appname(app_name);
    }
    if let Some(timeout) = payload.timeout {
        notification.timeout(notify_rust::Timeout::Milliseconds(timeout));
    }

    match notification.show() {
        Ok(_) => (http::StatusCode::CREATED, Json(serde_json::json!({"msg": "created success"}))),
        Err(_) => (http::StatusCode::BAD_REQUEST, Json(serde_json::json!({"msg": "created failed"})))
    }


}