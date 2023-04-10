use serde::Deserialize;

use crate::API_SURAH_ENDPOINT;

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
struct Surah {
    number: usize,
    englishName: String,
    englishNameTranslation: String,
    numberOfAyahs: usize,
    revelationType: String,
}

#[derive(Debug, Deserialize)]
pub struct Surahs {
    data: Vec<Surah>,
}

impl Surahs {
    pub async fn new() -> Self {
        let response = reqwest::get(API_SURAH_ENDPOINT).await.unwrap();

        response.json().await.unwrap()
    }

    pub fn print(&self) {
        for (index, element) in self.data.iter().enumerate() {
            println!(
                "[{:0>3}] {} | {}: {}, {} Ayahs",
                index + 1,
                element.englishName,
                element.englishNameTranslation,
                element.revelationType,
                element.numberOfAyahs
            );
        }
    }

    pub fn get_id(&self, place: usize) -> usize {
        self.data.get(place - 1).unwrap().number
    }
}
