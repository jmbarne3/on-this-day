mod services;

use clap::Parser;
use chrono::Utc;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long, default_value="all")]
    content_type: String,
    #[arg(short, long, default_value="en")]
    lang: String
}

fn main() {
    let args = Args::parse();

    let api = services::OnThisDayService::new(
        "https://api.wikimedia.org".to_string(),
        args.content_type,
        args.lang,
        Utc::now()
    );

    let _body = api.get_random_event();
}
