
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
type DBStore = Arc<Mutex<HashMap<String, HashMap<String, String>>>>;

#[derive(Debug, Clone)]
pub struct DataStore {
    store: DBStore,
}

impl DataStore {
    pub fn new() -> Self {
        DataStore {
            store: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn get_all(&self) -> Vec<(String, Vec<(String, String)>)>{
        let store = self.store.lock().unwrap();
        let mut result = HashMap::new();

        for (key, value) in store.iter() {
            result.insert(key.clone(), value.clone());
        }

        self.map_to_list(result)
    }

    pub fn upsert(&self, key: String, value: Vec<(String, String)>) {
        let mut store = self.store.lock().unwrap();
        
        let entry = store.entry(key).or_insert(HashMap::default());

        for (k, v) in value {
            entry.insert(k, v);
        }        
    }

    pub fn get(&self, key: String) -> Vec<(String, String)> {
        let store = self.store.lock().unwrap();
        let entry = store.get(&key);
        match entry {
            Some(value) => value.iter().map(|(k, v)| (k.clone(), v.clone())).collect(),
            None => Vec::new(),
        }
    }

    fn map_to_list(&self, map: HashMap<String, HashMap<String, String>>) -> Vec<(String, Vec<(String, String)>)> {
        let mut result: Vec<(String, Vec<(String, String)>)> = Vec::new();

        for (key, values) in map.iter() {
            result.push((key.clone(), values.iter().map(|(k, v)| (k.clone(), v.clone())).collect()));
        }

        result
    }
}