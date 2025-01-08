use std::collections::HashMap;
use std::fmt;

struct EventHandler<T> {
    listeners: HashMap<String, Vec<Box<dyn Fn(&T)>>>,
}

impl<T> EventHandler<T> {
    fn new() -> Self {
        Self {
            listeners: HashMap::new(),
        }
    }

    fn register(&mut self, event: &str, callback: Box<dyn Fn(&T)>) {
        self.listeners
            .entry(event.to_string())
            .or_insert(Vec::new())
            .push(callback);
    }

    fn unregister(&mut self, event: &str) {
        self.listeners.remove(event).unwrap();
    }

    fn emit(&self, event: &str, parameter: &T) {
        for (event_name, callback) in &self.listeners {
            if event_name == event || event_name == "*" {
                for cb in callback {
                    cb(parameter);
                }
            }
        }
    }
}

struct Data {
    name: String,
    value: i32,
}

impl fmt::Display for Data {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "name={}, value={}", self.name, self.value)
    }
}
