

use crate::helper_functions::{get_team_name, get_team_country};
use crate::team_requests::{search_for_team, get_team_stats};

use helper_functions::{print_sports_info, print_fixtures_info};
use reqwest::{Client, header::{}};
use search_response::{TeamSearchResponse, TeamInfo};
use fixture_response::{FixtureResponse, FixtureData};

mod secrets;
mod helper_functions;
mod search_response;
mod fixture_response;
mod team_requests;

pub static mut TEAM_ID: u32 = 0; // figure out safe way to use 

 fn main() {

    let team_name = get_team_name();

    println!();
    println!("Hello {}, fan ", team_name);

    let team_country = get_team_country();
    println!("");
    let _ = search_for_team(team_name, team_country);
    let _ = get_team_stats(unsafe { TEAM_ID }); // figure out safe way to use 
}

