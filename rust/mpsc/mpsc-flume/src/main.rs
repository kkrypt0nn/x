use std::{
    sync::{
        atomic::{AtomicU8, Ordering},
        Arc,
    },
    thread,
    thread::sleep,
    time::Duration,
};

use flume;

/// A basic enum for support of various events
enum Event {
    Ready,
    Number(u8, u8),
    TaskFinished,
    Finished,
}

const TOTAL_TASKS: u8 = 2;

fn main() {
    // Create a bounded channel to accept events
    let (tx, rx) = flume::bounded::<Event>(100);

    // Track the amount of finished tasks
    let finished_task_counter = Arc::new(AtomicU8::new(0));

    // Send the ready event
    tx.send(Event::Ready).unwrap();

    // Spawn tasks to emit numbers from 1 to 5
    for task_id in 1..=TOTAL_TASKS {
        let tx_clone = tx.clone();

        thread::spawn(move || {
            for i in 1..=5 {
                tx_clone.send(Event::Number(i, task_id)).unwrap();
                // Simulate work to check if tasks run concurrently
                sleep(Duration::from_millis(500 * task_id as u64));
            }
            // Notify that the task has finished
            tx_clone.send(Event::TaskFinished).unwrap();
        });
    }

    // Receive various events and handle them accordingly
    while let Ok(event) = rx.recv() {
        match event {
            Event::Ready => {
                println!("The event receiver is ready");
            }
            Event::Number(i, task_id) => {
                // Print the received number along with the task_id
                println!("Received {} from task {}", i, task_id);
            }
            Event::TaskFinished => {
                // Increment the finished task counter when a task finishes
                let count = finished_task_counter.fetch_add(1, Ordering::SeqCst) + 1;

                // Check if all tasks have finished processing
                // If yes then send a finished event
                if count == TOTAL_TASKS {
                    tx.send(Event::Finished).unwrap();
                }
            }
            Event::Finished => {
                println!("All events have been sent and the event handling is now finished");
                break;
            }
        }
    }
}
