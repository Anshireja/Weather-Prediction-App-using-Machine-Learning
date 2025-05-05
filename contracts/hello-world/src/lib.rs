#![no_std]
use soroban_sdk::{contract, contracttype, contractimpl, Env, Symbol, String, log, symbol_short};

#[contracttype]
#[derive(Clone)]
pub struct WeatherData {
    pub location: String,
    pub timestamp: u64,
    pub temperature: i32,
    pub rainfall_chance: u32,
}

const WEATHER_KEY: Symbol = symbol_short!("WTHR");

#[contract]
pub struct WeatherContract;

#[contractimpl]
impl WeatherContract {
    pub fn store_weather(env: Env, location: String, temperature: i32, rainfall_chance: u32) {
        let timestamp = env.ledger().timestamp();
        let data = WeatherData {
            location: location.clone(),
            timestamp,
            temperature,
            rainfall_chance,
        };
        env.storage().instance().set(&(WEATHER_KEY, location.clone(), timestamp), &data);
        log!(&env, "Weather data stored for {} at time {}", location, timestamp);
    }

    pub fn get_weather(env: Env, location: String, timestamp: u64) -> WeatherData {
        env.storage()
            .instance()
            .get(&(WEATHER_KEY, location.clone(), timestamp))
            .unwrap_or(WeatherData {
                location,
                timestamp,
                temperature: 0,
                rainfall_chance: 0,
            })
    }
}