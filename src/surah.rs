#![allow(unused)] // temporary

use soloud::*;
use std::thread;
use std::time::Duration;

use serde::Deserialize;

use crate::API_SURAH_ENDPOINT;

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
struct Ayah {
    audio: String,
    numberInSurah: usize,
    juz: usize,
    page: usize,
    sajda: bool,
}

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
pub struct Surah {
    number: usize,
    englishName: String,
    englishNameTranslation: String,
    numberOfAyahs: usize,
    revelationType: String,
    ayahs: Vec<Ayah>,
}

#[derive(Debug, Deserialize)]
struct Response {
    data: Surah,
}

impl Surah {
    pub async fn new(recitation_id: &str, surah_id: usize) -> Self {
        let response = reqwest::get(format!(
            "{}/{}/{}",
            API_SURAH_ENDPOINT, surah_id, recitation_id
        ))
        .await
        .unwrap();

        let temp_representation: Response = response.json().await.unwrap();

        temp_representation.data
    }

    pub async fn play(&self) {
        for (_index, element) in self.ayahs.iter().enumerate() {
            let sl = Soloud::default().unwrap();
            let mut wav = audio::Wav::default();

            let resp = reqwest::get(element.audio.as_str()).await.unwrap();
            let bytes = resp.bytes().await.unwrap();

            wav.load_mem(&bytes).unwrap();

            sl.play(&wav);
            while sl.voice_count() > 0 {
                thread::sleep(Duration::from_millis(100));
            }
        }
    }
}
