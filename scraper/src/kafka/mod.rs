use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::ClientConfig;
use crate::models::news::News;
use serde_json;

pub struct KafkaProducer {
    producer: FutureProducer,
    topic: String,
}

impl KafkaProducer {
    pub fn new(brokers: &str, topic: &str) -> Self {
        let producer: FutureProducer = ClientConfig::new()
            .set("bootstrap.servers", brokers)
            .create()
            .expect("Failed to create Kafka producer");

        Self {
            producer,
            topic: topic.to_string(),
        }
    }

    pub async fn send(&self, news: &News) {
        let payload = serde_json::to_string(news).unwrap();

        let record = FutureRecord::to(&self.topic)
            .payload(&payload)
            .key(&news.url);
          
        let _ = self.producer.send(record, std::time::Duration::from_secs(0)).await;
    }
}
