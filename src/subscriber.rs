// subscriber.rs

use rustdds::*;
use rustdds::no_key::{DataReader, DataSample};
use serde::{Deserialize};
use std::thread;
use std::time::Duration;

pub fn run_subscriber() {
    // DomainParticipant is always necessary
    let domain_participant = DomainParticipant::new(0).unwrap();

    let qos = QosPolicyBuilder::new()
        .reliability(policy::Reliability::Reliable { max_blocking_time: Duration::ZERO.into() })
        .build();

    // DDS Subscriber, only one is necessary for each thread (slight difference to
    // DDS specification)
    let subscriber = domain_participant.create_subscriber(&qos).unwrap();

    // Some DDS Topic that we can write and read from (basically only binds readers
    // and writers together)
    let some_topic = domain_participant.create_topic("some_topic".to_string(), "SomeType".to_string(), &qos, TopicKind::NoKey).unwrap();

    // Used type needs Deserialize for readers
    #[derive(Deserialize, Debug)]
    struct SomeType {
        data: String,
    }

    // Creating DataReader requires type and deserializer adapter (which is recommended to be CDR).
    // Reader needs to be mutable if any operations are used.
    let mut reader = subscriber
        .create_datareader_no_key::<SomeType, CDRDeserializerAdapter<SomeType>>(
            &some_topic,
            None)
        .unwrap();

    // Main thread for receiving data
    loop {
        println!("Waiting for data...");

        match reader.take_next_sample() {
            Ok(Some(data_sample)) => {
                let received_data = data_sample.value();
                println!("Received data: {:?}", received_data);
            },
            Ok(None) => {
                eprintln!("No data available");
            },
            Err(err) => {
                eprintln!("Error receiving data: {:?}", err);
            }
        }

        thread::sleep(Duration::from_secs(1)); // Sleep for 1 second
    }
}
