use std::{fmt, sync::Arc};

use dashmap::DashMap;
use anyhow::Result;

#[derive(Debug, Clone)]
pub struct Metric {
    data: Arc<DashMap<String, i64>>,
}

impl Metric {
    pub fn new() -> Self {
        Self {
            data: Arc::new(DashMap::new()),
        }
    }

    pub fn inc(&self, key: impl Into<String>) -> Result<()> {
        let mut count = self.data.entry(key.into()).or_insert(0);
        *count += 1;

        Ok(())
    }
}

impl fmt::Display for Metric {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for entry in self.data.iter() {
            write!(f, "{}: {}\n", entry.key(), entry.value())?;
        }
        Ok(())
    }
}