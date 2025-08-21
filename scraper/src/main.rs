use crate::{kafka::KafkaProducer, scrapers::{coin_telegraph::CoinTelegraphScraper, Scraper}};
use tokio::time::{sleep, Duration};

mod models;
mod scrapers;
mod logger;
mod http;
mod kafka;

#[tokio::main]
async fn main() {
  let scraper = CoinTelegraphScraper::new();
  let kafka = KafkaProducer::new("localhost:9092", "news-topic");

  loop {
    let news_list = scraper.execute().await;

    for news in news_list.iter() {
      kafka.send(news).await;
      info!(scraper.source(), "Envoyé sur Kafka: {}", news.title);
    }

    info!(scraper.source(), "Scraping terminé, {} articles trouvés.", news_list.len());

    sleep(Duration::from_secs(600)).await;
  }
}