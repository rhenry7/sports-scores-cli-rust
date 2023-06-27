
use reqwest::Client;
use std::collections::HashMap;
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


#[tokio::main]
async fn getStats(team_name: String) -> Result<(), reqwest::Error> {

    let client = Client::new();
    let api_secrets = Secrets::new();

    // Prepare headers as a HashMap
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(reqwest::header::USER_AGENT, api_secrets.key.parse().unwrap());

    println!("made it in the get stats function");
    // Prepare query parameters as a HashMap
    let mut params = HashMap::new();
    params.insert("league", "premiere league");
    params.insert("season", "2023");
    params.insert("team", &team_name);

    // Build the request
    let response = client
        .get("https://api.example.com/endpoint")
        .headers(headers)
        .query(&params)
        .send()
        .await?;

    // Check if the request was successful
    if response.status().is_success() {
        let response_text = response.text().await?;
        println!("Response: {}", response_text);
    } else {
        println!("Request failed with status code: {}", response.status());
    }

    Ok(())
}

 
fn getNames() -> String{
   let mut line = String::new();
   println!("Enter your team name :");
   std::io::stdin().read_line(&mut line).unwrap();
   return line;
}

 fn main() {
    // Take input from user
        // Take input from user

    // let mut team_name = String::new();
    // println!("Enter the name of your team: ");
    // println!("Hello , {}", team_name);
    let team_name = getNames();
    //let b1 = std::io::stdin().read_line(&mut line).unwrap();
     println!("Hello , {} fan", team_name);
    let _ = getStats(team_name);
    //let _ = getStats(team_name);
}

