
use reqwest::{Client, header::{AUTHORIZATION, CONTENT_TYPE, ACCEPT}};
use std::{collections::HashMap};
use serde::{Deserialize, Serialize};
use serde_json::value;
use std::env;

mod secrets;
use secrets::*;

#[derive(Debug, serde::Deserialize)]
struct ApiResponse {
    get: String,
    parameters: Parameters,
    errors: Vec<String>,
    results: u32,
    paging: Paging,
    response: Vec<TeamInfo>,
}

#[derive(Debug, serde::Deserialize)]
struct Parameters {
    search: String,
}

#[derive(Debug, serde::Deserialize)]
struct Paging {
    current: u32,
    total: u32,
}

#[derive(Debug, serde::Deserialize)]
struct TeamInfo {
    team: Team,
    venue: Venue,
}

#[derive(Debug, serde::Deserialize)]
struct Team {
    id: u32,
    name: String,
    code: Option<String>,
    country: String,
    founded: Option<u32>,
    national: bool,
    logo: String,
}

#[derive(Debug, serde::Deserialize)]
struct Venue {
    id: Option<u32>,
    name: Option<String>,
    address: Option<String>,
    city: Option<String>,
    capacity: Option<u32>,
    surface: Option<String>,
    image: Option<String>,
}


fn print_sports_info(teams: Vec<TeamInfo>, country: &str)  {// Borrow the value using a reference
    for team in teams {
        if team.team.country == country {
            {
                println!("team name: {}", team.team.name);
                println!("team id: {}", team.team.id);
                println!("team country: {}", team.team.country);
                let team_stats: String = team.team.name.chars().collect();
                println!("team stats: {}", team_stats);
                println!("---------");
            }
        } 
    }
}


#[tokio::main]
async fn getStats(team_name: String, country: String) -> Result<(), reqwest::Error> {

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

    match response.status() {
    reqwest::StatusCode::OK => {
        // on success, parse our JSON to an APIResponse
        match response.json::<ApiResponse>().await {
            Ok(parsed) => {
            print_sports_info(parsed.response, &country);
            },
            Err(parsed) => println!("Hm, the response didn't match the shape we expected. {:?}", parsed),
        };
    }
    reqwest::StatusCode::UNAUTHORIZED => {
        println!("Need to grab a new token");
    }
    other => {
        panic!("Uh oh! Something unexpected happened: {:?}", other);
    }
};

 Ok(())
}




 
fn get_team_name() -> String{
   let mut line = String::new();
   println!("Enter your team name: ");
   std::io::stdin().read_line(&mut line).unwrap();
   return line;
}

fn get_team_country() -> String{
   let mut line = String::new();
   println!("Enter your team country: ");
   std::io::stdin().read_line(&mut line).unwrap();
   return line.trim().to_string();
}

 fn main() {
    let team_name = get_team_name();
    let team_country = get_team_country();
    println!("Hello {}, fan ", team_name);
    let _ = getStats(team_name, team_country);
}

