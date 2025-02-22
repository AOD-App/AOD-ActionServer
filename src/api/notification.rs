use notify_rust::Notification;
use rocket::serde::{Deserialize, json::Json};
use rocket::post;

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct NotificationTemplate {
    summary: String,
    body: String,
    icon: String,
}


#[post("/notify", data="<notification>")]
pub async fn notify(notification: Json<NotificationTemplate>) {
    Notification::new()
        .summary(&notification.summary)
        .body(&notification.body)
        .icon(&notification.icon)
        .show()
        .unwrap();
}