// main.rs

mod publisher;
mod subscriber;

use std::thread;
use std::time::Duration;

fn main() {
    // Spawn a thread for running the subscriber
    let subscriber_handle = thread::spawn(|| {
        subscriber::run_subscriber();
    });

    // Spawn a thread for running the publisher
    let publisher_handle = thread::spawn(|| {
        publisher::run_publisher();
    });

    // Let both publisher and subscriber run indefinitely
    subscriber_handle.join().unwrap();
    publisher_handle.join().unwrap();
}
