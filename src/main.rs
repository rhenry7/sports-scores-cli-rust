
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use serde_json::value;
use std::env;

mod secrets;
use secrets::*;

struct GameStats {
    home_team: String,
    away_team: String,
    home_score: u32,
    away_score: u32,
    date: String,
}

fn main() {
    // Take input from user
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Enter the name of your team: ");
        return;
    }
    let team_name = &args[1];

    // GET request to API
    let client = Client::new();
    let api_values =  Secrets::new();
    let url = format!("https://api-football-v1.p.rapidapi.com/v3/timezone");  // Replace with the 
    println!("Hello, world!");
}
