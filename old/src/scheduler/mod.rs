use std::sync::OnceLock;

use crate::errors::Error;
use ractor::{Actor, ActorRef};

use crate::scheduler::definitions::{Scheduler, SchedulerMessage};

pub mod actor;
pub mod definitions;

static SCHEDULER: OnceLock<ActorRef<SchedulerMessage>> = OnceLock::new();

pub async fn init() -> miette::Result<(), Error> {
    let (cell, _) = Actor::spawn(None, Scheduler {}, ()).await?;
    let _ = SCHEDULER.set(cell);

    Ok(())
}

pub fn get() -> &'static ActorRef<SchedulerMessage> {
    SCHEDULER.get().expect("Scheduler not initialized")
}
