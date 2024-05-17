use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
    fmt,
};

use anyhow::{Result, anyhow};

#[derive(Debug, Clone)]
pub struct Metric {
    data: Arc<RwLock<HashMap<String, i64>>>,
}

impl Metric {
    pub fn new() -> Self {
        Self {
            data: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn inc(&self, key: impl Into<String>) -> Result<()> {
        let mut data = self
            .data.write()
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
            .read()
            .map_err(|e| anyhow!("can't get data from metrics: {}", e))?
            .clone()
        )
    }
}

impl fmt::Display for Metric {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let data = self.data.read().unwrap();
        for (key, value) in data.iter() {
            write!(f, "{}: {}\n", key, value)?;
        }
        Ok(())
    }
}