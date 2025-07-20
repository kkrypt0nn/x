use crate::event_bus::EventBus;

pub struct Session {
    pub bus: EventBus,
}

impl Session {
    pub fn new() -> Session {
        Session {
            bus: EventBus::new(),
        }
    }
}
