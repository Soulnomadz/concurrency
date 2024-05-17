use std::collections::HashMap;

#[derive(Debug)]
pub struct Metric {
    data: HashMap<String, i64>,
}

impl Metric {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    pub fn inc(&mut self, key: &str) {
        let count = self.data.entry(key.to_string()).or_insert(0);
        *count += 1;
    }

    pub fn dec(&mut self, key: &str) {
        let count = self.data.entry(key.to_string()).or_insert(0);
        *count -= 1;
    }

    pub fn snap(&self) -> HashMap<String, i64> {
        self.data.clone()
    }
}
