use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Updates {
    pub with_polling: bool,
    pub polling_interval: i32,
}
