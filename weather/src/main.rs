use std::io;
use serde::Deserialize;
use colored::*;
use core::str::FromStr;
use hifitime::prelude::*;

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
// API call, display, main loop
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

    let epoch = Epoch::from_gregorian_utc_hms(2015, 2, 7, 11, 22, 33);

    // Parsing with a custom format
    assert_eq!(
        Epoch::from_format_str("Sat, 07 Feb 2015 11:22:33", "%a, %d %b %Y %H:%M:%S").unwrap(),
        epoch
    );

    // And printing with a custom format
    let fmt = Format::from_str("%a, %d %b %Y %H:%M:%S").unwrap();
    assert_eq!(
        format!("{}", Formatter::new(epoch, fmt)),
        "Sat, 07 Feb 2015 11:22:33"
    );

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
    println!("{}", "Welcome to Weather Station!".bright_yellow()); // Displaying welcome message

    loop {
        println!("{}", "Please enter the name of the city:".bright_green()); // Prompting user to enter city name

        let mut city = String::new();
        io::stdin().read_line(&mut city).expect("Failed to read input"); // Reading user input for city name
        let city = city.trim();

        println!("{}", "Please enter the country code (e.g., US for United States):".bright_green()); // Prompting user to enter country code

        let mut country_code = String::new();
        io::stdin().read_line(&mut country_code).expect("Failed to read input"); // Reading user input for country code
        let country_code = country_code.trim();

        // Get your API key from OpenWeatherMap
        let api_key = ""; 

        // Calling the function to fetch weather information
        match get_weather_info(&city, &country_code, api_key) {
            Ok(response) => {
                display_weather_info(&response); // Displaying weather information
            }
            Err(err) => {
                eprintln!("Error: {}", err); // Printing error message in case of failure
            }
        }

        println!("{}", "Do you want to search for weather in another city? (yes/no):".bright_green()); // Prompting user to continue or exit
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input"); // Reading user input for continuation
        let input = input.trim().to_lowercase();

        if input != "yes" {
            println!("Thank you for using our software!");
            break; // Exiting the loop if user doesn't want to continue
        }
    }
}
