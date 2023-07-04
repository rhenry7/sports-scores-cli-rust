use reqwest::Client;

use crate::{secrets::Secrets, search_response::TeamSearchResponse, helper_functions::{print_sports_info, print_fixtures_info}, fixture_response::FixtureResponse};

#[tokio::main]
pub async fn search_for_team(team_name: String, country: String) -> Result<(), reqwest::Error> {

    let client = Client::new();
    let api_secrets = Secrets::new();
    let url = format!("https://api-football-v1.p.rapidapi.com/v3/teams?search={}", team_name);

    // Build the request 
    let response = client
        .get(url)
        .header("x-rapidapi-key", api_secrets.key)
        .header("x-rapidapi-host", api_secrets.host)
        .send()
        .await?;

    match response.status() {
    reqwest::StatusCode::OK => {
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
pub async fn get_team_stats(team_id: u32) -> Result<(), reqwest::Error> {

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

