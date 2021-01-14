use exitfailure::ExitFailure;
use reqwest::Url;
use serde_derive::{Deserialize, Serialize};
use structopt::StructOpt;
mod util;
use util::{kelvin_to_celsius, miles_per_sec_to_km};

#[derive(StructOpt)]
struct Cli {
    city: String,
    country_code: String,
    api_key: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Forecast {
    coord: Coord,
    weather: Weather,
    base: String,
    main: Temps,
    visibility: i32,
    wind: Wind,
    clouds: Clouds,
    dt: i32,
    sys: Sys,
    timezone: i32,
    id: i32,
    name: String,
    cod: i32,
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
    id: i32,
    main: String,
    description: String,
    icon: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Temps {
    temp: f64,
    feels_like: f64,
    temp_min: f64,
    temp_max: f64,
    pressure: i32,
    humidity: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Wind {
    speed: f64,
    deg: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Clouds {
    all: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Sys {
    r#type: f64,
    id: i32,
    country: String,
    sunrise: i32,
    sunset: i32,
}

impl Forecast {
    async fn get(place: (&String, &String), api_key: &String) -> Result<Self, ExitFailure> {
        let url = format!(
            "https://api.openweathermap.org/data/2.5/weather?q={},{}&appid={}",
            place.0, place.1, api_key
        );

        let url = Url::parse(&*url)?;

        let resp = reqwest::get(url).await?.json::<Forecast>().await?;
        Ok(resp)
    }
}

#[tokio::main]
async fn main() -> Result<(), ExitFailure> {
    let args = Cli::from_args();
    let place = (&args.city, &args.country_code);
    let resp = Forecast::get(place, &args.api_key).await?;
    let temp_cel = kelvin_to_celsius(resp.main.temp);
    let wind_speed = miles_per_sec_to_km(resp.wind.speed);

    println!(
        "{}:\nWeather: {}, Temp (celsius): {:.2}, Humidity: {}%, Wind Speed (km per sec): {:.2}",
        place.0, resp.weather.details.description, temp_cel, resp.main.humidity, wind_speed
    );

    Ok(())
}
