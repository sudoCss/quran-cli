use crate::{recitations::Recitations, surah::Surah, surahs::Surahs};

pub mod recitations;
pub mod surah;
pub mod surahs;

pub const API_EDITIONS_ENDPOINT: &str =
    "http://api.alquran.cloud/v1/edition?format=audio&language=ar&type=versebyverse";
pub const API_SURAH_ENDPOINT: &str = "http://api.alquran.cloud/v1/surah";

#[tokio::main]
async fn main() {
    let recitations = Recitations::new().await;
    recitations.print();
    let mut input = String::new();

    println!("Please choose one of the previous recitations by entering its number:");
    std::io::stdin().read_line(&mut input).unwrap();

    let place: usize = input.trim().parse().unwrap();

    let recitation_id = recitations.get_identifier(place);

    let surahs = Surahs::new().await;
    surahs.print();
    let mut input = String::new();

    println!("Please choose one of the previous surahs by entering its number:");
    std::io::stdin().read_line(&mut input).unwrap();

    let place: usize = input.trim().parse().unwrap();

    let surah_id = surahs.get_id(place);

    let surah = Surah::new(recitation_id, surah_id).await;
    surah.play().await;
}
