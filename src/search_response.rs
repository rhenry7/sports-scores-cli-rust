
use reqwest::{Client, header::{AUTHORIZATION, CONTENT_TYPE, ACCEPT}};
use std::{collections::HashMap};
use serde::{Deserialize, Serialize};


#[derive(Debug, serde::Deserialize)]
pub struct ApiResponse {
    get: String,
    parameters: Parameters,
    errors: Vec<String>,
    results: u32,
    paging: Paging,
    pub response: Vec<TeamInfo>,
}

#[derive(Debug, serde::Deserialize)]
pub struct Parameters {
    search: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct Paging {
    current: u32,
    total: u32,
}

#[derive(Debug, serde::Deserialize)]
pub struct TeamInfo {
    pub team: Team,
    pub venue: Venue,
}

#[derive(Debug, serde::Deserialize)]
pub struct Team {
    pub id: u32,
    pub name: String,
    pub code: Option<String>,
    pub country: String,
    founded: Option<u32>,
    national: bool,
    logo: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct Venue {
    id: Option<u32>,
    name: Option<String>,
    address: Option<String>,
    city: Option<String>,
    capacity: Option<u32>,
    surface: Option<String>,
    image: Option<String>,
}


