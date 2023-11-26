use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::UnboundedSender;
use uuid::Uuid;

use crate::api::rejections::Rejection;
use crate::types::SyncObject;

pub type Clients = SyncObject<HashMap<Uuid, Client>>;

pub struct Client {
    pub id: Uuid,
    pub sender: UnboundedSender<Result<warp::ws::Message, warp::Error>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Model {
    ENTITY,
    EVENT,
    STATE,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Message {
    FETCH {
        model: Model,
    },
    #[serde(skip_deserializing)]
    RESULT {
        data: String,
    },
    #[serde(skip_deserializing)]
    ERROR {
        error: Rejection,
    },
}

impl From<Rejection> for Message {
    fn from(value: Rejection) -> Self {
        Message::ERROR { error: value }
    }
}
