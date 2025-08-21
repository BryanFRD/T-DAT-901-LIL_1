use async_trait::async_trait;
use scraper::{Html, Selector};

use crate::{error, info, models::{news::News, source::Source}, scrapers::{Scraper, ScraperContext}};

pub struct CoinTelegraphScraper {
    ctx: ScraperContext,
}

impl CoinTelegraphScraper {
  pub fn new() -> Self {
    Self { ctx: ScraperContext::new(Source::CoinTelegraph, "https://cointelegraph.com") }
  }
}

#[async_trait]
impl Scraper for CoinTelegraphScraper {
  
  fn source(&self) -> Source {
    self.ctx.source.clone()
  }
  
  fn url(&self) -> &str {
    &self.ctx.url
  }
    
  async fn scrap(&self) -> Vec<News> {
    let mut news_list: Vec<News> = Vec::new();
    
    let html: String = match self.ctx.client.get(&self.ctx.url).send().await {
        Ok(resp) => match resp.text().await {
            Ok(text) => text,
            Err(e) => {
                error!(self.ctx.source.clone(), "Failed to read response text: {}", e);
                return news_list;
            }
        },
        Err(e) => {
            error!(self.ctx.source.clone(), "Failed to fetch CoinTelegraph: {}", e);
            return news_list;
        }
    };
    
    let document = Html::parse_document(&html);
    let selector = Selector::parse("a.post-card-inline__figure-link").unwrap();
    let article_urls: Vec<_> = document.select(&selector)
        .filter_map(|el| el.value().attr("href"))
        .map(|url| format!("https://cointelegraph.com{}", url))
        .collect();
      
    for url in article_urls {
      info!(self.ctx.source.clone(), "Scraping article: {}", url);
    }
    
    news_list
  }
}