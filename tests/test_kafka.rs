extern crate kafka;

pub const LOCAL_KAFKA_BOOTSTRAP_HOST: &'static str = "localhost:9092";
pub const TEST_TOPIC_NAME: &'static str = "kafka-rust-test";
pub const TEST_TOPIC_PARTITIONS: [i32; 2] = [0, 1];

mod client;
mod consumer_producer;
