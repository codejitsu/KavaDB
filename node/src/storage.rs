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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_in_memory_storage_put_and_read() {
        let mut storage = InMemoryStorage::new();
        storage
            .put("key1".to_string(), "value1".to_string())
            .unwrap();
        let value = storage.read(&"key1".to_string()).unwrap();
        assert_eq!(value, "value1");
    }

    #[test]
    fn test_in_memory_storage_read_key_by_range() {
        let mut storage = InMemoryStorage::new();
        storage
            .put("key1".to_string(), "value1".to_string())
            .unwrap();
        storage
            .put("key2".to_string(), "value2".to_string())
            .unwrap();
        storage
            .put("key3".to_string(), "value3".to_string())
            .unwrap();

        let result = storage
            .read_key_by_range(&"key1".to_string(), &"key2".to_string())
            .unwrap();

        let expected = vec![
            ("key1".to_string(), "value1".to_string()),
            ("key2".to_string(), "value2".to_string()),
        ];

        result.iter().for_each(|(k, v)| {
            assert_eq!(expected.contains(&(k.clone(), v.clone())), true);
        });
    }

    #[test]
    fn test_in_memory_storage_batch_put() {
        let mut storage = InMemoryStorage::new();
        let entries = vec![
            ("key1".to_string(), "value1".to_string()),
            ("key2".to_string(), "value2".to_string()),
        ];
        storage.batch_put(entries).unwrap();

        let value1 = storage.read(&"key1".to_string()).unwrap();
        let value2 = storage.read(&"key2".to_string()).unwrap();
        assert_eq!(value1, "value1");
        assert_eq!(value2, "value2");
    }

    #[test]
    fn test_in_memory_storage_delete() {
        let mut storage = InMemoryStorage::new();
        storage
            .put("key1".to_string(), "value1".to_string())
            .unwrap();
        storage.delete(&"key1".to_string()).unwrap();
        let result = storage.read(&"key1".to_string());
        assert!(result.is_err());
    }
}
