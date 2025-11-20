use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;

pub static KEY_STORE: Lazy<Arc<Mutex<HashMap<String, String>>>> =
    Lazy::new(|| Arc::new(Mutex::new(HashMap::new())));

pub fn add(key: String, value: String) {
    {
        let mut store = KEY_STORE.lock().unwrap();
        store.insert(key.clone(), value.clone());
    }
}

pub fn get(key: String) -> String {
    {
        let store = KEY_STORE.lock().unwrap();
        let val = store.get(&key);
        match val {
            Some(val) => val.clone(),
            None => String::from(""),
        }
    }
}

pub fn pop(key: String) {
    {
        let mut store = KEY_STORE.lock().unwrap();
        store.remove(&key);
    }
}
