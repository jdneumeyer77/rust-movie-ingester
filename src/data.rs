use chrono::NaiveDate;
use std::{collections::HashSet, fs::File, io::BufReader};

use crate::{parsing, Config};

// intermediate data structures after parsing and filtering data.
#[derive(Debug)]
pub struct Movie {
    pub id: String,
    pub genres: HashSet<String>,
    pub production_companies: HashSet<String>,
    pub release_date: NaiveDate,
    pub budget: i128,
    pub revenue: i128,
    pub profit: i128,
    pub avg_populatarity: f32,
    pub status: Status,
}

#[derive(PartialEq, Debug, Hash, Eq)]
pub enum Status {
    Released,
    Other,
}

impl Status {
    pub fn from_str(enum_str: &str) -> Status {
        if enum_str.is_empty() {
            Self::Other
        } else {
            match enum_str.to_lowercase().as_str() {
                "released" => Self::Released,
                _ => Self::Other,
            }
        }
    }
}

pub fn read_movie_metadata(file: &File, config: &Config) -> Vec<Movie> {
    let reader = BufReader::new(file);
    let mut reader = csv::ReaderBuilder::new()
        // .has_headers(true)
        // .trim(Trim::All)
        .from_reader(reader);

    let headers = reader.headers().unwrap().clone();

    reader
        .records()
        // .take(5)
        .flat_map(|x| x)
        .flat_map(|s| parsing::from_record(&s, &headers))
        .flat_map(|x| x.to_movie(&config.last_run))
        .collect()
}


mod tests;
