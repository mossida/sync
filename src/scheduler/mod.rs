use std::sync::OnceLock;

use ractor::{Actor, ActorRef};

use crate::scheduler::definitions::{Scheduler, SchedulerMessage};

pub mod actor;
pub mod definitions;

static SCHEDULER: OnceLock<ActorRef<SchedulerMessage>> = OnceLock::new();

pub async fn init() {
    let (cell, _) = Actor::spawn(None, Scheduler {}, ()).await.unwrap();
    SCHEDULER.set(cell).expect("Cannot initialize scheduler");
}

pub fn get() -> &'static ActorRef<SchedulerMessage> {
    SCHEDULER.get().expect("Scheduler not initialized")
}
