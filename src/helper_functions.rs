use reqwest::{Client, header::{}};
use search_response::{TeamSearchResponse, TeamInfo};
use fixture_response::{FixtureResponse, FixtureData};
use std::{};
use chrono::{TimeZone, Utc};

use crate::{search_response, fixture_response, TEAM_ID};

pub fn convert_timestamp(timestamp: u64) -> String {
        let dt = Utc.timestamp(timestamp.try_into().unwrap(), 0);
        dt.format("%-I:%M %p").to_string()
    }


pub fn print_sports_info(teams: Vec<TeamInfo>, country: &str) {// Borrow the value using a reference
    for team in teams {
        if team.team.country == country {
            {
                
                println!("team name: {}", team.team.name);
                println!("team id: {}", team.team.id);
                println!("team country: {}", team.team.country);
                println!("team country: {}", team.team.code);
                println!("---------");
            }
            unsafe { TEAM_ID = team.team.id };
            return; // only get the first team information 
           
        } 
    }
    
}

pub fn print_fixtures_info(teams: Vec<FixtureData>, team_id: u32) {// Borrow the value using a reference
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

pub fn get_team_name() -> String{
   let mut line = String::new();
   println!("Enter your team name: ");
   std::io::stdin().read_line(&mut line).unwrap();
   return line.trim().to_string();
}

pub fn get_team_country() -> String{
   let mut line = String::new();
   println!("Enter your team country: ");
   std::io::stdin().read_line(&mut line).unwrap();
   return line.trim().to_string();
}
