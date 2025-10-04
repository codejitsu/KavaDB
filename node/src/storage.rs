pub trait Storage {
    fn put(&mut self, key: String, value: String) -> Result<(), String>;
    fn read(&self, key: &String) -> Result<String, String>;
    fn read_key_by_range(
        &self,
        start: &String,
        end: &String,
    ) -> Result<Vec<(String, String)>, String>;
    fn batch_put(&mut self, entries: Vec<(String, String)>) -> Result<(), String>;
    fn delete(&mut self, key: &String) -> Result<(), String>;
}

pub struct InMemoryStorage {
    store: std::collections::HashMap<String, String>,
}

impl InMemoryStorage {
    fn new() -> Self {
        InMemoryStorage {
            store: std::collections::HashMap::new(),
        }
    }
}

impl Storage for InMemoryStorage {
    fn put(&mut self, key: String, value: String) -> Result<(), String> {
        self.store.insert(key, value);
        Ok(())
    }

    fn read(&self, key: &String) -> Result<String, String> {
        self.store
            .get(key)
            .cloned()
            .ok_or_else(|| "Key not found".to_string())
    }

    fn read_key_by_range(
        &self,
        start: &String,
        end: &String,
    ) -> Result<Vec<(String, String)>, String> {
        let mut result = Vec::new();
        for (key, value) in &self.store {
            if key >= start && key <= end {
                result.push((key.clone(), value.clone()));
            }
        }
        Ok(result)
    }

    fn batch_put(&mut self, entries: Vec<(String, String)>) -> Result<(), String> {
        for (key, value) in entries {
            self.store.insert(key, value);
        }
        Ok(())
    }

    fn delete(&mut self, key: &String) -> Result<(), String> {
        self.store
            .remove(key)
            .map(|_| ())
            .ok_or_else(|| "Key not found".to_string())
    }
}

pub struct StorageBuilder {
    storage_type: String,
}

impl StorageBuilder {
    pub fn builder(storage_type: &str) -> Self {
        match storage_type {
            "memory" => {
                return StorageBuilder {
                    storage_type: "memory".to_string(),
                };
            }
            _ => {
                eprintln!(
                    "Unknown storage type '{}', defaulting to 'memory'",
                    storage_type
                );
                return StorageBuilder {
                    storage_type: "memory".to_string(),
                };
            }
        }
    }

    pub fn build(self) -> Box<dyn Storage> {
        match self.storage_type.as_str() {
            "memory" => Box::new(InMemoryStorage::new()),
            _ => unreachable!(), // This should never happen due to the builder logic
        }
    }
}
