use dotenv_codegen::dotenv;
use exitfailure::ExitFailure;
use reqwest::Url;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Coord {
    pub lon: f64,
    pub lat: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Weather {
    pub details: Details,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Details {
    pub id: u32,
    pub main: String,
    pub description: String,
    pub icon: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Main {
    pub temp: f64,
    pub feels_like: f64,
    pub temp_min: f64,
    pub temp_max: f64,
    pub pressure: u32,
    pub humidity: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct W {
    pub coord: Coord,
    pub weather: Weather,
    pub base: String,
    pub main: Main,
}

impl W {
    pub async fn get(city: &String) -> Result<Self, ExitFailure> {
        let url = format!(
            "https://api.openweathermap.org/data/2.5/weather?q={}&appid={}",
            city,
            dotenv!("appid")
        );
        // println!("{}",url);
        let url = Url::parse(&*url)?;
        let resp = reqwest::get(url).await?.json::<W>().await?;
        Ok(resp)
    }
}
