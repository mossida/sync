use std::any::Any;
use std::collections::HashMap;

use log::info;
use once_cell::sync::Lazy;
use tokio::sync::{Mutex, MutexGuard};
use tokio::task::JoinHandle;

use crate::helpers::Helper;
use crate::integrations::{Adapter, Interface};
use crate::integrations::adapter::{AdapterId, AdapterManager};
use crate::integrations::interface::{InterfaceId, InterfaceManager};
use crate::types::{SyncMap, SyncObject};

static SCHEDULER: Lazy<Mutex<Scheduler>> = Lazy::new(|| Mutex::new(Scheduler::new()));

pub async fn get() -> MutexGuard<'static, Scheduler> {
    SCHEDULER.lock().await
}

pub struct Module<Instance, Collection: ?Sized + Any>(pub Instance, pub Collection);

#[derive(Default)]
pub struct Scheduler {
    handlers: HashMap<AdapterId, JoinHandle<()>>,
    wrappers: HashMap<AdapterId, Module<SyncObject<Adapter>, SyncMap<InterfaceId, Interface>>>,
}

impl Scheduler {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn setup<T>(&mut self, adapter: T)
    where
        T: AdapterManager + 'static,
    {
        let adapter_id = self.push_wrapper(adapter);
        let reference = self.wrappers.get(&adapter_id).unwrap().0.clone();

        tokio::spawn(async move {
            let mut guard = reference.lock().await;
            guard.setup().await;
        });

        self.execute();
    }

    pub fn register(&mut self, module: Module<AdapterId, impl InterfaceManager>) {
        let id = module.1.base().id;
        let reference = Helper::sync(module.1);

        self.wrappers.entry(module.0).and_modify(|m| {
            m.1.insert(id, reference);
        });
    }

    pub fn execute(&mut self) {
        info!("Execution from scheduler has been requested");
        for (key, module) in &self.wrappers {
            self.handlers.entry(*key).or_insert(tokio::spawn({
                let reference = module.0.clone();
                info!("Starting adapter: {key}");
                async move {
                    let guard = reference.lock().await;

                    loop {
                        guard.main().await;
                    }
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

    fn push_wrapper<T>(&mut self, adapter: T) -> AdapterId
    where
        T: AdapterManager + 'static,
    {
        let id = adapter.id();
        let reference = Helper::sync(adapter);
        self.wrappers
            .insert(id, Module(reference, Default::default()));

        return id;
    }
}
