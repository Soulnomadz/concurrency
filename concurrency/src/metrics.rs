use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use anyhow::{Result, anyhow};

#[derive(Debug, Clone)]
pub struct Metric {
    data: Arc<Mutex<HashMap<String, i64>>>,
}

impl Metric {
    pub fn new() -> Self {
        Self {
            data: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn inc(&self, key: impl Into<String>) -> Result<()> {
        let mut data = self
            .data.lock()
            .map_err(|e| anyhow!("inc error {:?}", e))?;

        let count = data.entry(key.into()).or_insert(0);
        *count += 1;

        Ok(())
    }

    // pub fn dec(&mut self, key: &str) {
    //     let count = self.data.entry(key.to_string()).or_insert(0);
    //     *count -= 1;
    // }

    pub fn snap(&self) -> Result<HashMap<String, i64>> {
        // self.data.clone()
        Ok(self
            .data
            .lock()
            .map_err(|e| anyhow!("can't get data from metrics: {}", e))?
            .clone()
        )
    }
}
