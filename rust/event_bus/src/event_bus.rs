use std::collections::HashMap;

pub enum Event {
    // String: content
    Message(String),

    SomethingElse,
}

impl std::fmt::Display for Event {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Message(_) => write!(f, "message"),
            Self::SomethingElse => write!(f, "something_else"),
        }
    }
}

type CallbackFn = Box<dyn Fn(&Event)>;

pub struct EventBus {
    subscribers: HashMap<String, Vec<CallbackFn>>,
}

impl EventBus {
    pub fn new() -> EventBus {
        EventBus {
            subscribers: HashMap::new(),
        }
    }

    pub fn subscribe<F>(&mut self, event_name: &str, callback: F)
    where
        F: Fn(&Event) + 'static,
    {
        self.subscribers
            .entry(event_name.to_string())
            .or_default()
            .push(Box::new(callback));
    }

    pub fn publish(&self, event: &Event) {
        if let Some(callbacks) = self.subscribers.get(event.to_string().as_str()) {
            for callback in callbacks {
                callback(event);
            }
        }
    }
}
