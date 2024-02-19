use serde::Deserialize;
use serde_json::Value;

use chrono::naive::NaiveDate;

#[derive(Deserialize, Debug, Clone)]
pub struct Listing {
    pub url: String,
    pub id: i32,
    pub city: String,
    pub city_area: String,
    pub street_name: String,
    pub postal_code: String,
    pub title: String,
    pub rooms: f32,
    pub size_m2: f32,
    pub monthly_rent: f32,
    pub deposit: f32,
    pub images: Value,
    pub available_from: Option<String>,
    pub advertised_date: String
}

impl Listing {
    pub fn human_friendly(&self) -> String {
        format!("
{title}
{size_m2} m2 {rooms} rooms
{monthly_rent} DKK
available: {available_from}
{city}, {city_area}
{street_name}, {postal_code}
https://www.boligportal.dk/{url}
        ", 
        title=self.title,
        size_m2=self.size_m2,
        rooms=self.rooms,
        monthly_rent=self.monthly_rent,
        available_from=self.available_from_as_string().unwrap_or("ASAPPP".to_string()),
        city=self.city,
        city_area=self.city_area,
        street_name=self.street_name,
        postal_code=self.postal_code,
        url=self.url)
    }

    pub fn available_from_as_string(&self) -> Option<String> {
        self.available_from.as_ref().map(|s| {
            let date = NaiveDate::parse_from_str(s, "%Y-%m-%d").unwrap();
            date.format("%d %B %Y").to_string()
        })
    }
}

#[derive(Deserialize, Debug)]
pub struct JsonResponse {
    pub offset: i32,
    pub limit: i32,
    pub result_count: i32,
    pub results: Vec<Listing>
}