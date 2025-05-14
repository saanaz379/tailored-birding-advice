use std::io;
use serde::Deserialize;
use colored::*;
use chrono::prelude::*;

//
// NEW DATA STRUCTURE for JSON response from bytes to readable text
//
#[derive(Deserialize, Debug)]
struct WeatherResponse {
    weather: Vec<Weather>,                              // description
    main: Main,                                         // temp, humidity, pressure
    wind: Wind,                                         // wind speed
    name: String,                                       // location name
}
#[derive(Deserialize, Debug)]
struct Weather {
    description: String,
}
#[derive(Deserialize, Debug)]
struct Main {
    temp: f64,
    humidity: f64,
    pressure: f64,
}
#[derive(Deserialize, Debug)]
struct Wind {
    speed: f64,
}

//
// API call, display
//
fn get_weather_info(city: &str, country_code: &str, api_key: &str) -> Result<WeatherResponse, reqwest::Error> {
    let url = format!(
        "http://api.openweathermap.org/data/2.5/weather?q={},{}&units=metric&appid={}",
        city, country_code, api_key
    );
    let response = reqwest::blocking::get(&url)?;
    let response_json = response.json::<WeatherResponse>()?;
    Ok(response_json)
}
fn display_weather_info(response: &WeatherResponse) {
    let description = &response.weather[0].description;
    let temperature = response.main.temp;
    let humidity = response.main.humidity;
    let pressure = response.main.pressure;
    let wind_speed = response.wind.speed;
    let weather_text = format!(
"Weather in {}: {} {}
> Temperature: {:.1}°C, 
> Humidity: {:.1}%, 
> Pressure: {:.1} hPa, 
> Wind Speed: {:.1} m/s",
        response.name,
        description,
        get_temperature_emoji(temperature),
        temperature,
        humidity,
        pressure,
        wind_speed,
    );
    let weather_text_colored = match description.as_str() {
        "clear sky" => weather_text.bright_yellow(),
        "few clouds" | "scattered clouds" | "broken clouds" => weather_text.bright_blue(),
        "overcast clouds" | "mist" | "haze" | "smoke" | "sand" | "dust" | "fog" | "squalls" => weather_text.dimmed(),
        "shower rain" | "rain" | "thunderstorm" | "snow" => weather_text.bright_cyan(),
        _ => weather_text.normal(),
    };
    println!("{}", weather_text_colored);

}
fn get_temperature_emoji(temperature: f64) -> &'static str {
    if temperature < 0.0 {
        "❄️"
    } else if temperature >= 0.0 && temperature < 10.0 {
        "☁️"
    } else if temperature >= 10.0 && temperature < 20.0 {
        "⛅"
    } else if temperature >= 20.0 && temperature < 30.0 {
        "🌤️"
    } else {
        "🔥"
    }
}
fn main() {
    println!("{}", "Welcome to your personal ornithologist! You will get advice tailored to your current weather and time of year to spot the most birds on your next expedition.".bright_yellow());

    println!("{}", "Please enter the name of the city you are currently in:".bright_green());

    let mut city = String::new();
    io::stdin().read_line(&mut city).expect("Failed to read input");
    let city = city.trim();

    println!("{}", "Please enter the country code you are currently in (e.g., US for United States):".bright_green());

    let mut country_code = String::new();
    io::stdin().read_line(&mut country_code).expect("Failed to read input");
    let country_code = country_code.trim();

    println!("{}", "Please enter your OpenWeatherMap API key. If you do not have one, fear not! You can still get advice tailored to your current season. Just type \"no\" in the field below.".bright_green());

    let mut api_key = String::new();
    io::stdin().read_line(&mut api_key).expect("Failed to read input");
    let api_key = api_key.trim();

    let local: DateTime<Local> = Local::now(); // e.g. `2014-11-28 21:45:59.324310806 -09:00`

    println!("Current time: {}", local);

    if api_key != "no" {
        // Calling the function to fetch weather information
        match get_weather_info(&city, &country_code, api_key) {
            Ok(response) => {
                display_weather_info(&response);
            }
            Err(err) => {
                eprintln!("Error: {}", err);
            }
        }
    }

    println!("{}", "Thank you for consulting with us! May the force be with you on your upcoming birdwatching expedition!!".bright_blue());

}