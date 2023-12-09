use hashbrown::HashMap;
use ractor::concurrency::{Duration, JoinHandle};
use ractor::{ActorCell, ActorRef};

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
