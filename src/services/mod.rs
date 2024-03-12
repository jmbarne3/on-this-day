use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Datelike, Utc};

#[derive(Serialize, Deserialize)]
struct WikiPage {
    pub r#type: String,
    pub title: String,
}

#[derive(Serialize, Deserialize)]
struct OTDObject {
    pub text: String,
    pub year: u32,
    pub pages: Vec<WikiPage>
}

#[derive(Serialize, Deserialize)]
struct OTDHoliday {
    pub text: String,
    pub pages: Vec<WikiPage>
}

#[derive(Serialize, Deserialize)]
struct OTDResponse {
    pub selected: Vec<OTDObject>,
    pub births: Vec<OTDObject>,
    pub deaths: Vec<OTDObject>,
    pub events: Vec<OTDObject>,
    pub holidays: Vec<OTDHoliday>,
}

pub struct OnThisDayService {
    pub api_base_url: String,
    pub c_type: String,
    pub language: String,
    pub date: DateTime<Utc>
}

impl OnThisDayService {
    pub fn new(api_base_url: String, c_type: String, language: String, date: DateTime<Utc>) -> OnThisDayService {
        return OnThisDayService{
            api_base_url: api_base_url,
            c_type: c_type,
            language: language,
            date: date
        };
    }

    pub fn get_random_event(&self) -> () {
        let url = format!("{}/feed/v1/wikipedia/{}/onthisday/{}/{:0>2}/{:0>2}",
            &self.api_base_url,
            &self.language,
            &self.c_type,
            &self.date.month().to_string(),
            &self.date.day().to_string()
        );

        let body = reqwest::blocking::get(&url)
            .expect("Something broke")
            .text()
            .expect("Something else went wrong");

        let results: OTDResponse = serde_json::from_str(&body).expect("Cannot parse the json");
        let mut rng = rand::thread_rng();

        let first = &results.selected.choose(&mut rng).expect("Problem getting random item");

        println!("\nOn this day in {}, {}:", self.date.format("%B %e"), first.year.to_string());
        println!("{}\n", first.text);

    }
}
