mod services;

// use std::env;
use chrono::Utc;

fn main() {
    let api = services::OnThisDayService::new(
        "https://api.wikimedia.org".to_string(),
        "en".to_string(),
        Utc::now()
    );

    let _body = api.get_random_event();
}
