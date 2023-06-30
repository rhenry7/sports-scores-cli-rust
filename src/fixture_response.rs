use reqwest::{Client, header::{AUTHORIZATION, CONTENT_TYPE, ACCEPT}};
use std::{collections::HashMap};
use serde::{Deserialize, Serialize};

#[derive(Debug, serde::Deserialize)]
pub struct FixtureResponse {
    pub get: String,
    pub parameters: Parameters,
    pub errors: Vec<String>,
    pub results: u32,
    pub paging: Paging,
    pub response: Vec<FixtureData>,
}

#[derive(Debug, serde::Deserialize)]
pub struct Parameters {
    pub season: String,
    pub team: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct Paging {
    pub current: u32,
    pub total: u32,
}

#[derive(Debug, serde::Deserialize)]
pub struct FixtureData {
    pub fixture: Fixture,
    pub league: League,
    pub teams: Teams,
    pub goals: Goals,
    pub score: Score,
}

#[derive(Debug, serde::Deserialize)]
pub struct Fixture {
    pub id: u32,
    pub referee: Option<String>,
    pub timezone: String,
    pub date: String,
    pub timestamp: u64,
    pub periods: Periods,
    pub venue: Venue,
    pub status: Status,
}

#[derive(Debug, serde::Deserialize)]
pub struct Periods {
    pub first: u64,
    pub second: u64,
}

#[derive(Debug, serde::Deserialize)]
pub struct Venue {
    pub id: Option<u32>,
    pub name: Option<String>,
    pub city: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
pub struct Status {
    pub long: String,
    pub short: String,
    pub elapsed: u8,
}

#[derive(Debug, serde::Deserialize)]
pub struct League {
    pub id: u32,
    pub name: String,
    pub country: String,
    pub logo: String,
    pub flag: Option<String>,
    pub season: u32,
    pub round: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct Teams {
    pub home: TeamInfo,
    pub away: TeamInfo,
}

#[derive(Debug, serde::Deserialize)]
pub struct TeamInfo {
    pub id: u32,
    pub name: String,
    pub logo: String,
    pub winner: Option<bool>,
}

#[derive(Debug, serde::Deserialize)]
pub struct Goals {
    pub home: u32,
    pub away: u32,
}

#[derive(Debug, serde::Deserialize)]
pub struct Score {
    pub halftime: Option<ScoreData>,
    pub fulltime: ScoreData,
    pub extratime: Option<ScoreData>,
    pub penalty: Option<ScoreData>,
}

#[derive(Debug, serde::Deserialize)]
pub struct ScoreData {
    pub home: Option<u32>,
    pub away: Option<u32>,
}
