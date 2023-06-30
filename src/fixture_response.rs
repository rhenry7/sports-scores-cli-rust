use reqwest::{Client, header::{AUTHORIZATION, CONTENT_TYPE, ACCEPT}};
use std::{collections::HashMap};
use serde::{Deserialize, Serialize};

#[derive(Debug, serde::Deserialize)]
pub struct Fixture {
    pub id: u32,
    pub referee: String,
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
    pub id: u32,
    pub name: String,
    pub city: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct Status {
    pub long: String,
    pub short: String,
    pub elapsed: u8,
}

fn main() {
    let fixture: Fixture = serde_json::from_str(r#"{
        "id": 592141,
        "referee": "K. Friend",
        "timezone": "UTC",
        "date": "2021-01-12T20:15:00+00:00",
        "timestamp": 1610482500,
        "periods": {
            "first": 1610482500,
            "second": 1610486100
        },
        "venue": {
            "id": 512,
            "name": "Turf Moor",
            "city": "Burnley"
        },
        "status": {
            "long": "Match Finished",
            "short": "FT",
            "elapsed": 90
        }
    }"#).unwrap();

    println!("{:#?}", fixture);
}
