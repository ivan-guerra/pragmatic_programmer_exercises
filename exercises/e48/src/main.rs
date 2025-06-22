//! # Weather Information Application
//!
//! This module implements an application that retrieves and displays current weather information
//! for a specified city in the United States using the OpenWeatherMap API.
//!
//! ## Features
//!
//! - **Location-based Weather**: Fetches weather data for user-specified locations
//! - **Temperature Display**: Shows temperature in both Fahrenheit and Celsius
//! - **Wind Direction**: Provides detailed wind direction using compass points
//! - **Weather Recommendations**: Suggests whether to bring an umbrella or wear a coat
//! - **Geocoding**: Converts city names to coordinates for accurate weather data
use anyhow::anyhow;
use serde::Deserialize;
use serde::Serialize;
use std::io::Write;

static OPENWEATHERMAP_API_KEY: &str = "680daa2576713c28bf8c20fd8fe7798b";

struct Location {
    city: String,
    state: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CurrentWeather {
    pub coord: Coord,
    pub weather: Vec<Weather>,
    pub base: String,
    pub main: Main,
    pub visibility: i64,
    pub wind: Wind,
    pub rain: Option<Rain>,
    pub clouds: Clouds,
    pub dt: i64,
    pub sys: Sys,
    pub timezone: i64,
    pub id: i64,
    pub name: String,
    pub cod: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Coord {
    pub lon: f64,
    pub lat: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Weather {
    pub id: i64,
    pub main: String,
    pub description: String,
    pub icon: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Main {
    pub temp: f64,
    pub feels_like: f64,
    pub temp_min: f64,
    pub temp_max: f64,
    pub pressure: i64,
    pub humidity: i64,
    pub sea_level: i64,
    pub grnd_level: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Wind {
    pub speed: f64,
    pub deg: i64,
    pub gust: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Rain {
    pub n1h: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Clouds {
    pub all: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Sys {
    #[serde(rename = "type")]
    pub type_field: i64,
    pub id: i64,
    pub country: String,
    pub sunrise: i64,
    pub sunset: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InvalidRequest {
    pub cod: String,
    pub message: String,
}

pub type GeocodeData = Vec<Root>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Root {
    pub name: String,
    pub local_names: Option<LocalNames>,
    pub lat: f64,
    pub lon: f64,
    pub country: String,
    pub state: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LocalNames {
    pub ur: Option<String>,
    pub hr: Option<String>,
    pub ar: Option<String>,
    pub rm: Option<String>,
    pub ak: Option<String>,
    pub ka: Option<String>,
    pub nn: Option<String>,
    pub te: Option<String>,
    pub ta: Option<String>,
    pub my: Option<String>,
    pub bs: Option<String>,
    pub co: Option<String>,
    pub yi: Option<String>,
    pub sv: Option<String>,
    pub sq: Option<String>,
    pub is: Option<String>,
    pub es: Option<String>,
    pub pa: Option<String>,
    pub ps: Option<String>,
    pub hy: Option<String>,
    pub so: Option<String>,
    pub an: Option<String>,
    pub be: Option<String>,
    pub th: Option<String>,
    pub zu: Option<String>,
    pub vi: Option<String>,
    pub fj: Option<String>,
    pub bm: Option<String>,
    pub mr: Option<String>,
    pub oc: Option<String>,
    pub cy: Option<String>,
    pub lt: Option<String>,
    pub bh: Option<String>,
    pub gv: Option<String>,
    pub tl: Option<String>,
    pub hi: Option<String>,
    pub pl: Option<String>,
    pub ru: Option<String>,
    pub ga: Option<String>,
    pub ko: Option<String>,
    pub kk: Option<String>,
    pub uk: Option<String>,
    pub wa: Option<String>,
    pub na: Option<String>,
    pub eu: Option<String>,
    pub ha: Option<String>,
    pub cs: Option<String>,
    pub os: Option<String>,
    pub ki: Option<String>,
    pub gn: Option<String>,
    pub ku: Option<String>,
    pub vo: Option<String>,
    pub mk: Option<String>,
    pub lv: Option<String>,
    pub ml: Option<String>,
    pub zh: Option<String>,
    pub la: Option<String>,
    pub sr: Option<String>,
    pub gd: Option<String>,
    pub ca: Option<String>,
    pub kn: Option<String>,
    pub ie: Option<String>,
    pub xh: Option<String>,
    pub mi: Option<String>,
    pub qu: Option<String>,
    pub fy: Option<String>,
    pub tr: Option<String>,
    pub sg: Option<String>,
    pub am: Option<String>,
    pub ia: Option<String>,
    pub tg: Option<String>,
    pub af: Option<String>,
    pub fi: Option<String>,
    pub ky: Option<String>,
    pub no: Option<String>,
    pub sh: Option<String>,
    pub it: Option<String>,
    pub ms: Option<String>,
    pub uz: Option<String>,
    pub id: Option<String>,
    pub yo: Option<String>,
    pub iu: Option<String>,
    pub ro: Option<String>,
    pub sl: Option<String>,
    pub rn: Option<String>,
    pub sn: Option<String>,
    pub az: Option<String>,
    pub eo: Option<String>,
    pub ug: Option<String>,
    pub fa: Option<String>,
    pub kl: Option<String>,
    pub gl: Option<String>,
    pub de: Option<String>,
    pub io: Option<String>,
    pub ne: Option<String>,
    pub kw: Option<String>,
    pub li: Option<String>,
    pub fo: Option<String>,
    pub sw: Option<String>,
    pub lb: Option<String>,
    pub et: Option<String>,
    pub mg: Option<String>,
    pub bn: Option<String>,
    pub sk: Option<String>,
    pub ja: Option<String>,
    pub nv: Option<String>,
    pub bi: Option<String>,
    pub mn: Option<String>,
    pub ig: Option<String>,
    pub da: Option<String>,
    pub sc: Option<String>,
    pub he: Option<String>,
    pub el: Option<String>,
    pub en: Option<String>,
    pub bg: Option<String>,
    pub br: Option<String>,
    pub pt: Option<String>,
    pub hu: Option<String>,
    pub tt: Option<String>,
    pub ht: Option<String>,
    pub tw: Option<String>,
    pub tk: Option<String>,
    pub jv: Option<String>,
    pub ce: Option<String>,
    pub st: Option<String>,
    pub se: Option<String>,
    pub nl: Option<String>,
    pub fr: Option<String>,
}

fn get_weather(location: &Coord) -> anyhow::Result<CurrentWeather> {
    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?lat={}&lon={}&units=imperial&appid={}",
        location.lat, location.lon, OPENWEATHERMAP_API_KEY
    );

    let response =
        reqwest::blocking::get(&url).map_err(|e| anyhow!("Failed to fetch weather data: {}", e))?;
    if !response.status().is_success() {
        let error_response = response.json::<InvalidRequest>()?;
        return Err(anyhow!(error_response.message));
    }

    Ok(response.json::<CurrentWeather>()?)
}

fn get_coord(location: &Location) -> anyhow::Result<Option<Coord>> {
    let url = format!(
        "http://api.openweathermap.org/geo/1.0/direct?q={},{},USA&limit=5&appid={}",
        location.city, location.state, OPENWEATHERMAP_API_KEY
    );

    let response = reqwest::blocking::get(&url)
        .map_err(|e| anyhow!("Failed to fetch geocoding data: {}", e))?;
    if !response.status().is_success() {
        let error_response = response.json::<InvalidRequest>()?;
        return Err(anyhow!(error_response.message));
    }

    let geocode_data = response.json::<GeocodeData>()?;
    geocode_data.first().map_or(Ok(None), |location| {
        Ok(Some(Coord {
            lon: location.lon,
            lat: location.lat,
        }))
    })
}

fn display_temp(weather: &CurrentWeather) {
    let temp_celsius = (weather.main.temp - 32.0) * 5.0 / 9.0;
    println!(
        "Current temperature in {}: {:.1}°F / {:.1}°C",
        weather.name, weather.main.temp, temp_celsius
    );
}

fn display_wind_direction(weather: &CurrentWeather) {
    let degrees = weather.wind.deg as f64;
    let direction = match degrees {
        348.75..=360.0 | 0.0..=5.62 => "North",
        5.63..=16.87 => "North by East",
        16.88..=28.12 => "North-Northeast",
        28.13..=39.37 => "Northeast by North",
        39.38..=50.62 => "Northeast",
        50.63..=61.87 => "Northeast by East",
        61.88..=73.12 => "East-Northeast",
        73.13..=84.37 => "East by North",
        84.38..=95.62 => "East",
        95.63..=106.87 => "East by South",
        106.88..=118.12 => "East-Southeast",
        118.13..=129.37 => "Southeast by East",
        129.38..=140.62 => "Southeast",
        140.63..=151.87 => "Southeast by South",
        151.88..=163.12 => "South-Southeast",
        163.13..=174.37 => "South by East",
        174.38..=185.62 => "South",
        185.63..=196.87 => "South by West",
        196.88..=208.12 => "South-Southwest",
        208.13..=219.37 => "Southwest by South",
        219.38..=230.62 => "Southwest",
        230.63..=241.87 => "Southwest by West",
        241.88..=253.12 => "West-Southwest",
        253.13..=264.37 => "West by South",
        264.38..=275.62 => "West",
        275.63..=286.87 => "West by North",
        286.88..=298.12 => "West-Northwest",
        298.13..=309.37 => "Northwest by West",
        309.38..=320.62 => "Northwest",
        320.63..=331.87 => "Northwest by North",
        331.88..=343.12 => "North-Northwest",
        343.13..=348.74 => "North by West",
        _ => "Unknown",
    };

    println!("Wind direction: {}", direction);
}

fn recommend_umbrella(weather: &CurrentWeather) -> bool {
    let has_weather = !weather.weather.is_empty();
    let has_rain = ["rain", "drizzle", "thunderstorm"].iter().any(|w| {
        weather
            .weather
            .iter()
            .any(|weather| weather.main.to_lowercase() == *w)
    });

    has_weather && has_rain
}

fn recommend_coat(weather: &CurrentWeather) -> bool {
    weather.main.temp < 60.0
}

fn prompt_for_location() -> anyhow::Result<Location> {
    let mut city = String::new();
    let mut state = String::new();

    print!("Enter the city name: ");
    std::io::stdout().flush()?;
    std::io::stdin().read_line(&mut city)?;
    let city = city.trim().to_string();

    print!("Enter the state abbreviation (e.g., CA): ");
    std::io::stdout().flush()?;
    std::io::stdin().read_line(&mut state)?;
    let state = state.trim().to_string();

    Ok(Location { city, state })
}

fn main() -> anyhow::Result<()> {
    let location = prompt_for_location()?;
    let coord = get_coord(&location)?.ok_or_else(|| {
        anyhow!(
            "Could not find coordinates for '{}' in '{}'.",
            location.city,
            location.state
        )
    })?;
    let weather = get_weather(&coord)?;

    display_temp(&weather);
    display_wind_direction(&weather);
    if recommend_umbrella(&weather) {
        println!("You might need an umbrella today.");
    }
    if recommend_coat(&weather) {
        println!("You might need a coat today.");
    }

    Ok(())
}
