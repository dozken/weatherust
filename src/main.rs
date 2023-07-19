use clap::Parser;
use serde::Deserialize;
use serde::Serialize;
use std::error::Error;

const LAT: f32 = 1.3286777392194342;
const LON: f32 = 103.91334535620457;

#[derive(Parser)]
#[command(name = "wet")]
#[command(about = "Weather in your terminal", long_about = None)]
struct Args {
    // Number of days for the forecast
    #[arg(short, default_value_t = 0)]
    days: u8,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct WeatherData {
    pub cod: String,
    pub message: i32,
    pub cnt: i32,
    pub list: Vec<WeatherList>,
    pub city: City,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WeatherList {
    pub dt: i64,
    pub main: Main,
    pub weather: Vec<Weather>,
    pub clouds: Clouds,
    pub wind: Wind,
    pub visibility: i32,
    pub pop: f32,
    pub rain: Option<Rain>,
    pub sys: Sys,
    pub dt_txt: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Main {
    pub temp: f32,
    pub feels_like: f32,
    pub temp_min: f32,
    pub temp_max: f32,
    pub pressure: i32,
    pub sea_level: i32,
    pub grnd_level: i32,
    pub humidity: i32,
    pub temp_kf: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Weather {
    pub id: i32,
    pub main: String,
    pub description: String,
    pub icon: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Clouds {
    pub all: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Wind {
    pub speed: f32,
    pub deg: i32,
    pub gust: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Rain {
    #[serde(rename = "3h")]
    pub three_h: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Sys {
    pub pod: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct City {
    pub id: i32,
    pub name: String,
    pub coord: Coord,
    pub country: String,
    pub population: i32,
    pub timezone: i32,
    pub sunrise: i64,
    pub sunset: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Coord {
    pub lat: f32,
    pub lon: f32,
}

fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv().unwrap(); // !!

    let mut api_key = None;
    for (key, value) in std::env::vars() {
        if key == "WEATHER_API_KEY" {
            api_key = Some(value);
        }
    }
    let api_key = api_key.unwrap();

    let args = Args::parse();

    let days = args.days;
    let method = match days {
        0 => "weather",
        1 => "onecall",
        _ => "forecast",
    };

    let cnt = days;

    let url: String = format!("https://api.openweathermap.org/data/2.5/{method}?lat={LAT}&lon={LON}&unitis=metric&cnt={cnt}&APPID={api_key}");
    let result: WeatherData = reqwest::blocking::get(&url)?.json()?;
    result
        .list
        .into_iter()
        .flat_map(|weather| {
            println!("{}", weather.dt_txt);
            return weather.weather.into_iter();
        })
        .for_each(|weather| println!("{:?}", weather));

    Ok(())
}
