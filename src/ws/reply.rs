use crate::api::rejections::Rejection;
use crate::ws::models::{Client, Message};
use serde::Serialize;

pub fn error(client: &Client, rejection: Rejection) {
    let text = serde_json::to_string::<Message>(&rejection.into()).unwrap();
    let _ = client.sender.send(Ok(warp::ws::Message::text(text)));
}

pub fn result<T: Serialize>(client: &Client, data: T) {
    let data = serde_json::to_string(&data).unwrap();
    let text = serde_json::to_string(&Message::RESULT { data }).unwrap();
    let _ = client.sender.send(Ok(warp::ws::Message::text(text)));
}
