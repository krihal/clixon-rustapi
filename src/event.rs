use log::{debug, error, info};
use std::collections::HashMap;
use std::fmt;
use wildmatch::WildMatch;

pub struct EventHandler<T> {
    listeners: HashMap<String, Vec<Box<dyn Fn(&T)>>>,
}

impl<T> EventHandler<T> {
    pub fn new() -> Self {
        Self {
            listeners: HashMap::new(),
        }
    }

    pub fn register(&mut self, event: &str, callback: Box<dyn Fn(&T)>) {
        self.listeners
            .entry(event.to_string())
            .or_insert(Vec::new())
            .push(callback);

        debug!("Registered new callback for \"{}\"", event);
    }

    pub fn unregister(&mut self, event: &str) {
        if self.listeners.contains_key(event) {
            self.listeners.remove(event);
        }

        debug!("Unregistered new callback for \"{}\"", event);
    }

    pub fn emit(&self, event: &str, parameter: &T) {
        for (event_name, callback) in &self.listeners {
            if WildMatch::new(event_name).matches(event) {
                debug!("Emitting event for {}", event);
                for cb in callback {
                    cb(parameter);
                }
            }
        }
    }
}

pub struct Data {
    name: String,
    value: i32,
}

impl fmt::Display for Data {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "name={}, value={}", self.name, self.value)
    }
}
