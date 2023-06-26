
use chrono::NaiveDate;
use csv::StringRecord;
use serde::Deserialize;
use std::collections::HashSet;

use crate::{Status, MovieRow};

#[derive(Debug, Deserialize)]
pub struct MovieRowRaw {
    id: String,
    genres: String,
    production_companies: String,
    release_date: NaiveDate,
    budget: i128,
    revenue: i128,
    #[serde(deserialize_with = "csv::invalid_option", rename = "popularity")]
    avg_populatarity: Option<f32>,
    status: String,
}

pub fn from_record(record: &StringRecord, headers: &StringRecord) -> Result<MovieRowRaw, csv::Error> {       
    record.deserialize(Some(&headers))
}

impl MovieRowRaw {
    // TODO: This should probably return result instead.
    pub fn validated_movie_row(&self, last_run: &Option<NaiveDate>) -> Option<MovieRow> {
        // TODO: only status == released is valid at the moment.
        if self.revenue > 0
           && last_run.map(|x| self.release_date <= x).unwrap_or(true)
            && Status::from_str(&self.status) == Status::Released
        {
            // pull genres and production companies.
            Some(MovieRow {
                id: self.id.clone(),
                genres: convert_json_to_set(&self.genres),
                production_companies: convert_json_to_set(&self.production_companies),
                release_date: self.release_date,
                budget: self.budget,
                revenue: self.revenue,
                avg_populatarity: self.avg_populatarity.unwrap_or(0.0),
                status: Status::Released,
                profit: self.revenue - self.budget,
            })
        } else {
            None
        }
    }
}

fn convert_json_to_set(s: &str) -> HashSet<String> {
    let s = s.replace("'", "\"");
    //println!("raw: {}", s);
    let parsed = json::parse(&s);

    parsed
        .map(|v| {
            v.members()
                .flat_map(|obj| 
                    obj["id"].as_i32().map(|x| x.to_string())
                )
                .collect()
        })
        .unwrap_or(HashSet::new())
}

