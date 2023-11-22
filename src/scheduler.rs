use std::any::Any;
use std::collections::HashMap;
use std::ops::Deref;

use once_cell::sync::Lazy;
use tokio::task::JoinHandle;

use crate::helpers::Helper;
use crate::integrations::adapter::{AdapterId, AdapterManager};
use crate::integrations::interface::{InterfaceId, InterfaceManager};
use crate::integrations::{Adapter, Interface};
use crate::types::{SyncMap, SyncObject};

static SCHEDULER: Lazy<Scheduler> = Lazy::new(|| Scheduler::new());

pub fn get() -> &'static Scheduler {
    SCHEDULER.deref()
}

struct Module<Instance, Collection: ?Sized + Any>(Instance, Collection);

#[derive(Default)]
pub struct Scheduler {
    handlers: HashMap<AdapterId, JoinHandle<()>>,
    wrappers: HashMap<AdapterId, Module<SyncObject<Adapter>, SyncMap<InterfaceId, Interface>>>,
}

impl Scheduler {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn setup<T: AdapterManager>(&mut self, adapter: T) {
        let rc = self.push_wrapper(adapter).clone();

        tokio::spawn(async move {
            let mut guard = rc.lock().await;
            guard.setup().await;
        });

        self.execute();
    }

    pub fn register(&mut self, module: Module<AdapterId, impl InterfaceManager>) {
        let reference = Helper::sync(module.1);

        self.wrappers.entry(module.0).and_modify(|m| {
            m.1.insert(module.1.base().id, reference);
        });
    }

    pub fn execute(&mut self) {
        for (key, module) in &self.wrappers {
            self.handlers
                .entry(*key)
                .or_insert(tokio::spawn(async move {
                    let guard = module.0.lock().await;

                    loop {
                        guard.main().await;
                    }
                }));
        }
    }
}

/** Private functions */
impl Scheduler {
    fn stop(&mut self) {
        self.handlers.drain().for_each(|h| h.1.abort());
    }

    fn push_wrapper<T: AdapterManager>(&mut self, adapter: T) -> SyncObject<T> {
        let reference = Helper::sync(adapter);
        self.wrappers
            .insert(adapter.id(), Module(reference, Default::default()));

        reference
    }
}
