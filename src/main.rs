use std::io;
use serde::Deserialize;
use colored::*

//struct to deserialize the JSON response from openweathermap API
#[derive(Deserialize, Debug)]

struct WeatherResponse {
    main: Main,
    weather: Vec<Weather>,
    wind: Wind,
    name: String,
}

// struct to represent the main weather data
#[derive(Deserialize, Debug)]
struct Main {
    description: String,
}

/ Struct to represent the weather parameters
#[derive(Deserialize, Debug)]
struct main {
    temp: f64,
    pressure: f64,
    humidity: f64,
}

// Struct to represent wind information
#[derive(Deserialize, Debug)]
struct Wind {
    speed: f64,
}

// function to get the weather data from the API
fn get_weather_info(city: &str, country_code: &str api_key: &str) -> Result<WeatherResponse, reqwest::Error> {
    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?q={}&appid={}&units=metric", city, country_code, api_key
        city, api_key
    );
    let response = reqwest::blocking::get(&url)?;
    let response_json: WeatherResponse = response.json()?;
    Ok(response_json);
}

// Function to display the weather information
fn display_weather_info(weather: &WeatherResponse) {
    // extract te weather information from the response
    let description = &string = &response.weather[0].description;
    let temperature = &f64 = response.main.temp;
    let pressure = &f64 = response.main.pressure;
    let humidity = &f64 = response.main.humidity;
    let wind_speed = &f64 = response.wind.speed;

    //formating weather information into a string

    let weather_text: string = format!(
        "Weather in {}: {} {}
        Temperature: {:.1}°C,
        Pressure: {:.1} hPa,
        Humidity: {:.1}%
        Wind Speed: {:.1} m/s",
        response.name,
        description,
        get_temp_emoji(temperature),
        temperature,
        humidity,
        pressure,
        wind_speed,
    );

    // Coloring the weather text based on weather conditions
    let weather_text_colored: coloredString = match description.as_str() {
        "clear sky" => weather_text.bright_yellow(),
        "few clouds" | "scattered clouds" | "broken clouds" => 
        weather_text.bright_blue(),
        "overcast clouds" | "mist" | "haze" | "smoke" | "sand" | 
        "dust" | "fog" | "squalls" = > weather_text.dimmed(),
        "shower rain" | "rain" | "thunderstorm" | "snow" => weather_text.bright_cyan(),
        _ => weather_text.normal(),
    };
    //print the colored weather information
    println!("{}", weather_text_colored);
}

    // function to get the emoji based on temperature
    fn get_temp_emoji(temperature: f64) -> &str {
        if temperature < 0.0 {
            "❄️"
        } else if temperature >= 0.0 && temperature < 10.0 {
            "🌬️"
        } else if temperature >= 10.0 && temperature < 20.0 {
            "🌤️"
        } else if temperature >= 20.0 && temperature < 30.0 {
            "☀️"
        } else {
            "🔥"
        }
        
    }

    fn main(){
        println!("{}", "Welcome to Weather Station!".bright_yellow());
        loop {
            //Reading City
            println!("{}", "Please enter the name of the city:".bright_green());
            let mut city = String::new();
            io::stdin().read_line(&mut city).expect("Failed to read line");
            let city = &str = city.trim();

            //Reading Country
            println!("{}", "Please enter the country code (e.g. , US for United States)".bright_green());
            let mut country_code = String::new();
            let mut country_code = &str = country_code.trim();
            io::stdin().read_line(&mut country_code).expect("Failed to read line");
            let country_code = &str = country_code.trim();

            //Get your api key from openweathermap
            let api_key = "279c9c2028096ded701534da1fa7dc3b";

            //calling the function to fetch weather information
            match get_weather_info(city, country_code, api_key) {
                ok (response) => {
                    display_weather_info(&response);
                }
                Err(e) => {
                    println!("Error fetching weather data: {}", e);
                }
            }

            println!("{}", "Do you want to search for weather in another city? (y/n)".bright_green());
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read input");
            let input = input.trim().to_lowercase();
            if input != "y" {
                println!("{}", "Thank you for using Weather Station!".bright_yellow());
                break;
            }
        }
    }