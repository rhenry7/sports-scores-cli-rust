
use reqwest::{Client, header::{AUTHORIZATION, CONTENT_TYPE, ACCEPT}};
use search_response::{TeamSearchResponse, TeamInfo};
use fixture_response::{FixtureResponse, FixtureData};
use std::{collections::HashMap, time};
use serde::{Deserialize, Serialize};
use serde_json::value;
use std::env;
use chrono::{DateTime, TimeZone, Utc};


mod secrets;
use secrets::*;
mod search_response;
mod fixture_response;

static mut TEAM_ID: u32 = 0;

fn convert_timestamp(timestamp: u64) -> String {
    let dt = Utc.timestamp(timestamp.try_into().unwrap(), 0);
    dt.format("%-I:%M %p").to_string()
}

fn print_sports_info(teams: Vec<TeamInfo>, country: &str) {// Borrow the value using a reference
    for team in teams {
        if team.team.country == country {
            {
                
                println!("team name: {}", team.team.name);
                println!("team id: {}", team.team.id);
                println!("team country: {}", team.team.country);
                println!("---------");
            }
            unsafe { TEAM_ID = team.team.id };
            return; // only get the first team information 
           
        } 
    }
    
}

fn print_fixtures_info(teams: Vec<FixtureData>, team_id: u32) {// Borrow the value using a reference
    for team in teams {
        if team.teams.home.id == team_id  || team.teams.home.id == team_id {
            {
                
                println!("Match time: {}", convert_timestamp(team.fixture.timestamp));
                println!("Match day: {}", team.fixture.date);
                println!("Match location: {:#?}", team.fixture.venue.name);
                println!("{}: {}", team.teams.home.name, team.goals.home);
                println!("{}: {}", team.teams.away.name, team.goals.away);
                println!("{}, Season: {}", team.league.name, team.league.season);
                println!("Referee: {:?}", team.fixture.referee);
                println!("---------");
            }        
        }
    }
    
}

#[tokio::main]
async fn search_for_team(team_name: String, country: String) -> Result<(), reqwest::Error> {

    let client = Client::new();
    let api_secrets = Secrets::new();
    let url = format!("https://api-football-v1.p.rapidapi.com/v3/teams?search={}", team_name);

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
        match response.json::<TeamSearchResponse>().await {
                     
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

#[tokio::main]
async fn get_team_stats(team_id: u32) -> Result<(), reqwest::Error> {

    let client = Client::new();
    let api_secrets = Secrets::new();
    let url = format!("https://api-football-v1.p.rapidapi.com/v3/fixtures?season=2022&team={}", team_id);

    // Build the request 
    let response = client
        .get(url)
        .header("x-rapidapi-key", api_secrets.key)
        .header("x-rapidapi-host", api_secrets.host)
        //.query(&params)
        .send()
        .await?;

    // leave to test when wanting to see the shape of the response
    // let response_text = response.text().await?;
    // println!("Response body: {}", response_text);
    
    match response.status() {
    reqwest::StatusCode::OK => {
        match response.json::<FixtureResponse>().await {
            Ok(parsed) => {
            print_fixtures_info(parsed.response, team_id);
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
   return line.trim().to_string();
}

fn get_team_country() -> String{
   let mut line = String::new();
   println!("Enter your team country: ");
   std::io::stdin().read_line(&mut line).unwrap();
   return line.trim().to_string();
}

 fn main() {
    let team_name = get_team_name();
    println!("");
    println!("Hello {} fan ", team_name);
    println!("");
    let team_country = get_team_country();
    println!("");
    let _ = search_for_team(team_name, team_country);
    let _ = get_team_stats(unsafe { TEAM_ID });
}

