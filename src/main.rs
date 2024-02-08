// use std::collections::HashMap;
use scraper::{Html, Selector};

use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize, Debug)]
struct Listing {
    url: String,
    id: i32,
    city: String,
    city_area: String,
    street_name: String,
    postal_code: String,
    title: String,
    rooms: f32,
    size_m2: f32,
    monthly_rent: f32,
    deposit: f32,
    images: Value,
    available_from: Option<String>,
    advertised_date: String
}

#[derive(Deserialize, Debug)]
struct PageProps {
    offset: i32,
    limit: i32,
    result_count: i32,
    results: Vec<Listing>
}

#[derive(Deserialize, Debug)]
struct Props {
    page_props: PageProps
}

#[derive(Deserialize, Debug)]
struct JsonStore {
    props: Props
}



#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let html = reqwest::get("https://www.boligportal.dk/en/rental-apartments,rental-houses,rental-townhouse/k%C3%B8benhavn/?max_monthly_rent=12000&min_size_m2=40")
        .await?
        .text()
        .await?;
    // println!("{html:#?}");
    let document = Html::parse_document(&html);

    let selector = Selector::parse("#store").unwrap();

    let json_str = document.select(&selector).next().unwrap().inner_html();
    
    let j: JsonStore = serde_json::from_str(&json_str).unwrap();

    
    j.props.page_props.results.iter().for_each(|listing| {
        if listing.available_from.is_none() {
            println!("{:?}", listing);
        }
    });

    Ok(())
}