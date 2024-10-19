use std::{
    sync::{
        atomic::{AtomicU8, Ordering},
        Arc,
    },
    thread::sleep,
    time::Duration,
};

use tokio::sync::mpsc;

/// A basic enum for support of various events
enum Event {
    Ready,
    Number(u8, u8),
    TaskFinished,
    Finished,
}

const TOTAL_TASKS: u8 = 2;

#[tokio::main]
async fn main() {
    // Create a bounded channel to accept events
    let (tx, mut rx) = mpsc::channel::<Event>(100);

    // Track the amount of finished tasks
    let finished_task_counter = Arc::new(AtomicU8::new(0));

    // Send the ready event
    tx.send(Event::Ready).await.unwrap();

    // Spawn tasks to emit numbers from 1 to 5
    for task_id in 1..=TOTAL_TASKS {
        let tx_clone = tx.clone();

        tokio::spawn(async move {
            for i in 1..=5 {
                tx_clone.send(Event::Number(i, task_id)).await.unwrap();
                // Simulate work to check if tasks run concurrently
                sleep(Duration::from_millis(500 * task_id as u64));
            }
            // Notify that the task has finished
            tx_clone.send(Event::TaskFinished).await.unwrap();
        });
    }

    // Receive various events and handle them accordingly
    while let Some(event) = rx.recv().await {
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
                    tx.send(Event::Finished).await.unwrap();
                }
            }
            Event::Finished => {
                println!("All events have been sent and the event handling is now finished");
                break;
            }
        }
    }
}
