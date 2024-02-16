use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
pub struct Categories {
    pub values: Vec<String>
}

#[derive(Serialize, Debug)]
pub struct CityLevel {
    pub values: Vec<String>
}

#[derive(Serialize, Debug)]
pub struct BoligportalSearchPayload {
    pub categories: Categories,
    pub city_level_1: CityLevel,
    // pub city_level_2: CityLevel,
    // pub city_level_3: CityLevel,
    // pub rooms: Rooms,
    pub min_size_m2: i32,
    pub max_monthly_rent: i32,
    pub order: String,
    // pub min_rental_period: Option<String>,
    // pub max_available_from: String,
    // pub company_filter_key: Option<String>,
    // pub company_key: Option<String>,
    // pub street_name: StreetName,
    // pub social_housing: Option<String>,
    // pub min_lat: Option<String>,
    // pub min_lng: Option<String>,
    // pub max_lat: Option<String>,
    // pub max_lng: Option<String>,
    // pub shareable: bool,
    // pub furnished: bool,
    // pub student_only: bool,
    // pub pet_friendly: bool,
    // pub balcony: bool,
    // pub senior_friendly: bool,
    // pub parking: bool,
    // pub elevator: bool,
    // pub electric_charging_station: Option<String>,
    // pub dishwasher: Option<String>,
    // pub dryer: Option<String>,
    // pub washing_machine: Option<String>,
}

pub struct BoligportalQueryParams {
    pub offset: i32,
}