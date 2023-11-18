use std::sync::Arc;

use tokio::sync::Mutex;

use crate::integrations::adapter::AdapterManager;
use crate::integrations::interface::InterfaceManager;

type ScheduleRecipient<T> = Arc<Mutex<T>>;

pub struct Scheduler {
    pub(crate) adapters: Vec<Arc<Mutex<dyn AdapterManager>>>,
}

impl Scheduler {
    pub fn schedule_adapter_setup<T>(&mut self, adapter: T)
    where
        T: AdapterManager + 'static,
    {
        let recipient: ScheduleRecipient<T> = Arc::new(Mutex::new(adapter));

        tokio::spawn({
            let _recipient = recipient.clone();
            async move {
                let mut _guard = _recipient.lock().await;
                _guard.setup().await;
            }
        });

        self.adapters.push(recipient.clone())
    }

    pub fn schedule_update<T>(&self, recipient: ScheduleRecipient<T>)
    where
        T: InterfaceManager + 'static,
    {
        tokio::spawn(async move {
            // Will be completed as soon as possible
            let mut _guard = recipient.lock().await;
            _guard.update().await;
        });
    }
}
