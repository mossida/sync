use serde::Serialize;

use crate::api::rejections::Rejection;
use crate::ws::models::{MessageId, ReplyType, ResponseMessage, Sender};

pub fn error(id: MessageId, sender: &Sender, rejection: Rejection) {
    let mut message: ResponseMessage<Rejection> = rejection.into();
    message.id = id;
    let text = serde_json::to_string::<ResponseMessage<Rejection>>(&message).unwrap();
    let _ = sender.send(warp::ws::Message::text(text));
}

pub fn result<T: Serialize>(id: MessageId, sender: &Sender, data: T) {
    let text = serde_json::to_string(&ResponseMessage {
        id,
        r#type: ReplyType::Result,
        data,
    })
    .unwrap();

    let _ = sender.send(warp::ws::Message::text(text));
}
