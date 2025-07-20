use crate::{event_bus::Event, session::Session};

pub struct MessageListener {}

impl MessageListener {
    pub fn new(session: &mut Session) -> MessageListener {
        session.bus.subscribe("message", |event| {
            if let Event::Message(content) = event {
                println!("[MessageListener] Got message: {}", content);
            }
        });
        MessageListener {}
    }
}

pub struct SomethingElse {}

impl SomethingElse {
    pub fn new(session: &mut Session) -> SomethingElse {
        session.bus.subscribe("something_else", |event| {
            if let Event::SomethingElse = event {
                println!("[SomethingElse] Got event");
            }
        });
        SomethingElse {}
    }
}
