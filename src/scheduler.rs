use std::collections::HashMap;

use tokio::spawn;
use tokio::task::JoinHandle;

use crate::helpers::Helper;
use crate::integrations::adapter::{AdapterId, AdapterManager};
use crate::integrations::interface::InterfaceManager;
use crate::types::{SyncMap, SyncObject};

pub struct Scheduler {
    pub(crate) runners: HashMap<AdapterId, JoinHandle<()>>,
    pub adapters: SyncMap<AdapterId, dyn AdapterManager>,
}

impl Scheduler {
    pub fn stop(&mut self) {
        for (_, runner) in &self.runners {
            runner.abort();
        }
    }

    pub fn run(&mut self) {
        for (id, recipient) in &self.adapters {
            self.runners.entry(*id).or_insert(spawn({
                let _recipient = recipient.clone();
                async move {
                    let mut _guard = _recipient.lock().await;
                    loop {
                        _guard.main().await;
                    }
                }
            }));
        }
    }

    pub async fn setup_adapter<T>(&mut self, adapter: T)
    where
        T: AdapterManager + 'static,
    {
        self.stop(); // Interrupt the current execution

        let adapter_id = adapter.base().id;
        let recipient: SyncObject<T> = Helper::create_sync_object(adapter);

        let _ = tokio::join!({
            let _recipient = recipient.clone();
            async move {
                let mut _guard = _recipient.lock().await;
                _guard.setup().await;
            }
        });

        self.adapters.insert(adapter_id, recipient.clone());
        self.run(); // Run again with the new adapter
    }

    pub fn schedule_update<T>(recipient: SyncObject<T>)
    where
        T: InterfaceManager + 'static + ?Sized,
    {
        spawn(async move {
            let mut _guard = recipient.lock().await;
            _guard.update().await;
        });
    }
}
