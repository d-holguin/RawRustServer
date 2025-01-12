use std::{
    collections::HashMap,
    hash::Hash,
    sync::{Arc, Mutex},
};

use crate::error::Result;

pub struct SimpleDB<K, V>
where
    K: Eq + Hash + Clone,
    V: Clone,
{
    storage: Arc<Mutex<HashMap<K, V>>>,
}

impl<K, V> SimpleDB<K, V>
where
    K: Eq + Hash + Clone,
    V: Clone,
{
    pub fn new() -> Self {
        Self {
            storage: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    pub fn insert(&self, key: K, value: V) -> Result<()> {
        let mut storage = self
            .storage
            .lock()
            .map_err(|e| format!("Failed inserting to database: {}", e.to_string()))?;

        storage.insert(key, value);
        Ok(())
    }
    pub fn get(&self, key: K) -> Result<Option<V>> {
        let storage = self
            .storage
            .lock()
            .map_err(|e| format!("Failed reading from database: {}", e.to_string()))?;
        let value = storage.get(&key).cloned();
        Ok(value)
    }
    pub fn update(&self, key: K, value: V) -> Result<Option<V>> {
        let mut storage = self
            .storage
            .lock()
            .map_err(|_| "Failed updating database")?;
        let old_value = storage.insert(key, value);
        Ok(old_value)
    }

    pub fn remove(&self, key: K) -> Result<Option<V>> {
        let mut storage = self
            .storage
            .lock()
            .map_err(|_| "Failed removing from database")?;
        let removed_value = storage.remove(&key);
        Ok(removed_value)
    }
}
