use derive_more::{From, Into};
use hashbrown::HashMap;
use ractor::concurrency::{Duration, JoinHandle};
use ractor::{ActorCell, ActorRef};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Id;

use crate::integrations::classes::Class;

#[derive(Debug, From, Into, Serialize, Deserialize, Clone)]
pub struct InterfaceName(String);

impl InterfaceName {
    pub fn new(adapter_name: String, class: Class) -> Self {
        InterfaceName(format!(
            "{}/{}/{}",
            adapter_name,
            Id::rand(),
            class.to_string()
        ))
    }
}

// Messages

pub enum SchedulerMessage {
    Ping,
    RequestPolling(Duration, ActorRef<InterfaceMessage>),
    StopPolling(ActorCell),
}

pub enum InterfaceMessage {
    Update,
}

pub enum AdapterMessage {
    Action(String), // Calls an action on the adapter,
}

// Actor

pub struct Scheduler {}

pub struct SchedulerState {
    pollers: HashMap<String, JoinHandle<()>>,
}

impl SchedulerState {
    pub fn new() -> Self {
        SchedulerState {
            pollers: Default::default(),
        }
    }

    pub fn add_poller(&mut self, id: String, poller: JoinHandle<()>) {
        self.pollers.entry(id).or_insert_with(|| poller);
    }

    pub fn remove_poller(&mut self, id: String) -> Option<JoinHandle<()>> {
        self.pollers.remove(&id)
    }
}
