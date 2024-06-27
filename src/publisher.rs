// publisher.rs

use rustdds::*;
use rustdds::no_key::{DataWriter};
use serde::{Serialize};
use std::thread;
use std::time::Duration;

pub fn run_publisher() {
    // DomainParticipant is always necessary
    let domain_participant = DomainParticipant::new(0).unwrap();

    let qos = QosPolicyBuilder::new()
        .reliability(policy::Reliability::Reliable { max_blocking_time: Duration::ZERO.into() })
        .build();

    // DDS Publisher, only one is necessary for each thread (slight difference to
    // DDS specification)
    let publisher = domain_participant.create_publisher(&qos).unwrap();

    // Some DDS Topic that we can write and read from (basically only binds readers
    // and writers together)
    let some_topic = domain_participant.create_topic("some_topic".to_string(), "SomeType".to_string(), &qos, TopicKind::NoKey).unwrap();

    // Used type needs Serialize for writers
    #[derive(Serialize, Debug, Clone)]
    struct SomeType {
        data: String,
    }

    // Creating DataWriter required type and serializer adapter (which is recommended to be CDR).
    let writer = publisher
        .create_datawriter_no_key::<SomeType, CDRSerializerAdapter<SomeType>>(
            &some_topic,
            None)
        .unwrap();

    // Spawn a thread for publishing data
    let publish_handle = thread::spawn(move || {
        let some_data = SomeType { data: "Hello, DDS!".to_string() };
        loop {
            // Print data before sending
            println!("Sending data: {:?}", some_data);

            // This should send the data to all who listen "some_topic" topic.
            writer.write(some_data.clone(), None).unwrap();

            thread::sleep(Duration::from_secs(1)); // Sleep for 1 second
        }
    });

    // Join the publishing thread to avoid it being detached
    publish_handle.join().unwrap();
}
