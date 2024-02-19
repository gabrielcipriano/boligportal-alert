use serde::Serialize;

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

impl BoligportalSearchPayload {
    pub fn default() -> BoligportalSearchPayload {
        BoligportalSearchPayload {
            categories: Categories {
                values: vec![
                    "rental_apartment".to_string(),
                    "rental_house".to_string(),
                    "rental_townhouse".to_string()
                ]
            },
            city_level_1: CityLevel {
                values: vec!["københavn".to_string()]
            },
            min_size_m2: 39,
            max_monthly_rent: 12000,
            order: "DEFAULT".to_string()
        }
    }

    pub fn to_string(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

pub struct BoligportalQueryParams {
    pub offset: i32,
}

impl BoligportalQueryParams {
    pub fn new(offset: i32) -> BoligportalQueryParams {
        BoligportalQueryParams {
            offset
        }
    }

    pub fn to_params_tuples(&self) -> Vec<(&str, String)> {
        vec![
            ("offset", self.offset.to_string())
        ]
    }
}