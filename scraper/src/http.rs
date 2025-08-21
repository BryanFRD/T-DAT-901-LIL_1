use reqwest::Client;
use once_cell::sync::Lazy;

pub static HTTP_CLIENT: Lazy<Client> = Lazy::new(|| {
  Client::builder()
    .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.3")
    .timeout(std::time::Duration::from_secs(10))
    .build()
    .unwrap()
});