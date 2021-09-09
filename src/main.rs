use clap::{load_yaml, App};
use exitfailure::ExitFailure;
use reqwest::Url;
use serde_derive::{Deserialize, Serialize};
use structopt::StructOpt;

// mod tools;

#[derive(StructOpt)]
struct Input {
    city: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Coord {
    lon: f64,
    lat: f64,
}

#[derive(Serialize, Deserialize, Debug)]
struct Weather {
    details: Details,
}

#[derive(Serialize, Deserialize, Debug)]
struct Details {
    id: u32,
    main: String,
    description: String,
    icon: String,
}
#[derive(Serialize, Deserialize, Debug)]
struct Main {
    temp: f64,
    feels_like: f64,
    temp_min: f64,
    temp_max: f64,
    pressure: u32,
    humidity: u32,
}

#[derive(Serialize, Deserialize, Debug)]
struct W {
    coord: Coord,
    weather: Weather,
    base: String,
    main: Main,
}

impl W {
    async fn get(city: &String) -> Result<Self, ExitFailure> {
        let url = format!(
            "https://api.openweathermap.org/data/2.5/weather?q={}
        &appid=3c0cf4a46a0856f5e0e9958bed9c1206",
            city
        );
        let url = Url::parse(&*url)?;
        let resp = reqwest::get(url).await?.json::<W>().await?;
        Ok(resp)
    }
}
#[tokio::main]
async fn main() -> Result<(), ExitFailure> {
    let yaml = load_yaml!("../conf/cli.yml");
    let m = App::from_yaml(yaml).get_matches();

    let input_city = m.value_of("city").unwrap_or("Beijing").to_string();
    // let input = Input::from_args();
    // println!("city : {}", input.city);
    // let resp = W::get(&input.city).await?;
    let resp = W::get(&input_city).await?;
    // println!("{:#?}",resp);
    println!("天气 : {}", resp.weather.details.description);
    // println!("最低温 : {:.2}",tools::transform(resp.main.temp_min));
    // println!("最高温 : {:.2}",tools::transform(resp.main.temp_max));
    Ok(())
}
