use serde::Serialize;

use crate::errors::Error;
use crate::ws::models::{MessageId, ReplyType, ResponseMessage, Sender};

pub fn error(id: MessageId, sender: &Sender, err: Error) {
    let mut message: ResponseMessage<String> = err.into();
    message.id = id;
    let text = serde_json::to_string::<ResponseMessage<String>>(&message).unwrap();
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
