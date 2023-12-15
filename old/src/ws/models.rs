use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::UnboundedSender;

use crate::errors::Error;
use crate::ws::reply::{error, result};
use crate::{devices, entities};

pub type MessageId = u16;
pub type Sender = UnboundedSender<warp::ws::Message>;

#[derive(Serialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum ReplyType {
    Result,
    Error,
}

#[derive(Serialize, Debug)]
pub struct ResponseMessage<T: Serialize> {
    pub r#type: ReplyType,
    pub id: MessageId,
    pub data: T,
}

impl From<Error> for ResponseMessage<String> {
    fn from(value: Error) -> Self {
        ResponseMessage {
            id: 0,
            r#type: ReplyType::Error,
            data: value.to_string(),
        }
    }
}

#[async_trait]
#[typetag::serde(tag = "type")]
pub trait MessageHandler: Send + Sync {
    fn id(&self) -> MessageId;
    async fn handle(&self, sender: &Sender);
}

#[derive(Serialize, Deserialize, Debug)]
struct RequestDevicesMessage {
    id: MessageId,
}

#[derive(Serialize, Deserialize, Debug)]
struct RequestEntitiesMessage {
    id: MessageId,
}

#[async_trait]
#[typetag::serde(name = "get/devices")]
impl MessageHandler for RequestDevicesMessage {
    fn id(&self) -> MessageId {
        self.id
    }

    async fn handle(&self, sender: &Sender) {
        match devices::api::list_all().await {
            Ok(data) => result(self.id, sender, data),
            Err(err) => error(self.id, sender, err),
        };
    }
}

#[async_trait]
#[typetag::serde(name = "get/entities")]
impl MessageHandler for RequestEntitiesMessage {
    fn id(&self) -> MessageId {
        self.id
    }

    async fn handle(&self, sender: &Sender) {
        match entities::api::fetch_all().await {
            Ok(data) => result(self.id, sender, data),
            Err(err) => error(self.id, sender, err),
        };
    }
}
