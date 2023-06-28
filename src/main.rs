
use reqwest::{Client, header::{AUTHORIZATION, CONTENT_TYPE, ACCEPT}};
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
    // let mut headers = reqwest::header::HeaderMap::new();
    // headers.insert(reqwest::header::USER_AGENT, api_secrets.key.parse().unwrap());
    // headers.insert(reqwest::header::USER_AGENT, api_secrets.host.parse().unwrap());

    let url = format!("https://api-football-v1.p.rapidapi.com/v3/teams?search={}", team_name);

    // Prepare query parameters as a HashMap
    // let mut params = HashMap::new();
    // params.insert("country", "England");
    // params.insert("season", "2023");


    // Build the request 
    let response = client
        .get(url)
        .header("x-rapidapi-key", api_secrets.key)
        .header("x-rapidapi-host", api_secrets.host)
        //.query(&params)
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
    let team_name = getNames();
    print!("Hello , {} fan", team_name);
    let _ = getStats(team_name);
}

