use clap::{load_yaml, App};
use exitfailure::ExitFailure;
use weather_CLI::W;

#[tokio::main]
async fn main() -> Result<(), ExitFailure> {
    let yaml = load_yaml!("../conf/cli.yml");
    let m = App::from_yaml(yaml).get_matches();

    let input_city = m.value_of("city").unwrap_or("Beijing").to_string();
    let resp = W::get(&input_city).await?;
    println!("weather : {}", resp.weather.details.description);
    Ok(())
}
