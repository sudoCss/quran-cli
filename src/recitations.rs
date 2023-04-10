use serde::Deserialize;

use crate::API_EDITIONS_ENDPOINT;

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
struct Recitation {
    identifier: String,
    englishName: String,
}

#[derive(Debug, Deserialize)]
pub struct Recitations {
    data: Vec<Recitation>,
}

impl Recitations {
    pub async fn new() -> Self {
        let response = reqwest::get(API_EDITIONS_ENDPOINT).await.unwrap();

        response.json().await.unwrap()
    }

    pub fn print(&self) {
        for (index, element) in self.data.iter().enumerate() {
            println!("[{:0>3}] {}", index + 1, element.englishName);
        }
    }

    pub fn get_identifier(&self, place: usize) -> &str {
        self.data.get(place - 1).unwrap().identifier.as_str()
    }
}
