pub mod coin_telegraph;

use async_trait::async_trait;
use crate::{http::HTTP_CLIENT, models::{news::News, source::Source}};

pub struct ScraperContext {
    client: &'static reqwest::Client,
    source: Source,
    url: String,
}

impl ScraperContext {
    pub fn new(source: Source, url: &str) -> Self {
        Self { 
          client: &*HTTP_CLIENT, 
          source, 
          url: url.to_string() 
      }
    }
}

#[async_trait]
pub trait Scraper {
  
  fn source(&self) -> Source;
  fn url(&self) -> &str;
  async fn scrap(&self) -> Vec<News>;
  
  async fn execute(&self) -> Vec<News> {
    // crate::metrics::start_prometheus_server();

    let start = std::time::Instant::now();

    let news = self.scrap().await;

    let duration = start.elapsed();
    //crate::metrics::SCRAP_DURATION.observe(duration.as_secs_f64());
    //crate::metrics::SCRAPED_NEWS_TOTAL.inc_by(news.len() as u64);

    //crate::logger::info!(self.source(), &format!("Scraping terminé en {:?}", duration));

    news
  }
}