use std::{thread, time::Duration};

use event_bus::Event;
use session::Session;
use subscribers::{MessageListener, SomethingElse};

mod event_bus;
mod session;
mod subscribers;

fn main() {
    let mut session = Session::new();

    let _message_listener = MessageListener::new(&mut session);
    let _something_else = SomethingElse::new(&mut session);

    let mut counter = 1;
    loop {
        session
            .bus
            .publish(&Event::Message("Hello world".to_string()));
        if counter % 5 == 0 {
            session.bus.publish(&Event::SomethingElse);
        }

        counter += 1;
        thread::sleep(Duration::from_secs(1));
    }
}
