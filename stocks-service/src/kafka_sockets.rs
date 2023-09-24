use std::sync::Mutex;
use lazy_static::lazy_static;
use rdkafka::config::RDKafkaLogLevel;
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::ClientConfig;

lazy_static! {
    static ref KAFKA_BROKER: String =
        std::env::var("KAFKA_BROKER").expect("Can't read Kafka broker address");
    static ref KAFKA_TOPIC: String =
        std::env::var("KAFKA_TOPIC").expect("Can't read Kafka topic name");
}

pub fn create_consumer(group_id: String) -> StreamConsumer {
    let consumer: StreamConsumer = ClientConfig::new()
        .set("group.id", &group_id)
        .set("bootstrap.servers", KAFKA_BROKER.as_str())
        .set("enable.partition.eof", "false")
        .set("session.timeout.ms", "6000")
        .set("enable.auto.commit", "true")
        .set_log_level(RDKafkaLogLevel::Debug)
        .create()
        .expect("Consumer creation failed");

    consumer
        .subscribe(&[&KAFKA_TOPIC])
        .expect("Can't subscribe to specified topics");

    consumer
}

pub fn get_kafka_consumer_group_id(kafka_consumer_counter: &Mutex<i32>) -> String {
    let mut counter = kafka_consumer_counter.lock().expect("Can't lock counter");
    *counter += 1;
    format!("graphql-group-{}", *counter)
}