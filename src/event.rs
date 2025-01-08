use std::collections::HashMap;

type Callback = Box<dyn Fn()>;

struct EventHandler {
    listeners: HashMap<String, Vec<Callback>>,
}

impl EventHandler {
    fn new() -> Self {
        Self {
            listeners: HashMap::new(),
        }
    }

    fn register(&mut self, event: &str, callback: Callback) {
        self.listeners
            .entry(event.to_string())
            .or_insert(Vec::new())
            .push(callback);
    }

    fn emit(&self, event: &str) {
        for (event_name, callback) in &self.listeners {
            if event_name != event && event_name != "*" {
                continue;
            }

            for cb in callback {
                println!("Firing callback for event '{}'", event);
                cb();
            }
        }
    }
}
