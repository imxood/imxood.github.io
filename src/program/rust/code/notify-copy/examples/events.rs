use event_listener::Event;
use std::{sync::Arc, thread, time::Duration};

fn main() {
    let event = Arc::new(Event::new());

    // Spawn a thread that will set the flag after 1 second.
    thread::spawn({
        let event = event.clone();
        move || {
            loop {
                thread::sleep(Duration::from_secs(1));
                println!("nitify event");
                // Notify all listeners that the flag has been set.
                event.notify(usize::MAX);
            }
        }
    });

    loop {
        // Wait for a notification and continue the loop.
        event.listen().wait();
        println!("recv event");
    }
}
