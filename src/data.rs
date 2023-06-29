use chrono::NaiveDate;
use kstring::KString;
use std::{collections::HashSet, fs::File, io::BufReader};

use crate::{parsing, Config};

// intermediate data structures after parsing and filtering data.
#[derive(Debug)]
pub struct Movie {
    pub id: KString,
    pub genres: HashSet<i64>,
    pub production_companies: HashSet<i64>,
    pub release_date: NaiveDate,
    pub budget: i64,
    pub revenue: i64,
    pub profit: i64,
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
