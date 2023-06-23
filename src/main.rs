use chrono::{NaiveDate, ParseError};
use csv::{StringRecord, Trim, ErrorKind};
use fs::File;
use serde::Deserialize;
use std::{collections::HashSet, env, error::Error, fmt::Display, fs, io::BufReader, borrow::BorrowMut};


#[derive(Debug, Deserialize)]
struct MovieRowRaw {
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

#[derive(Debug)]
struct MovieRow {
    id: String,
    genres: HashSet<String>,
    production_companies: HashSet<String>,
    release_date: NaiveDate,
    budget: i128,
    revenue: i128,
    profit: i128,
    avg_populatarity: f32,
    status: String,
}

impl MovieRowRaw {
    fn validated_movie_row(&self, last_run: &Option<NaiveDate>) -> Option<MovieRow> {
        if self.revenue > 0
           && last_run.map(|x| self.release_date <= x).unwrap_or(true)
            && self.status.to_lowercase().contains("released")
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
                status: self.status.clone(),
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

fn from_record(record: &StringRecord, headers: &StringRecord) -> Result<MovieRowRaw, csv::Error> {       
    record.deserialize(Some(&headers))
}

fn main() {
    let args = env::args();
    let config = parse_args(args.into_iter().by_ref());

    let file = File::open(&config.input_file).expect("Couldn't read file...");
    println!("opened file for reading: {}", &config.input_file);
    let reader = BufReader::new(file);
    let mut reader = csv::ReaderBuilder::new()
   // .has_headers(true)
   // .trim(Trim::All)
    .from_reader(reader);
    println!("created reader!");

    let headers = reader.headers().unwrap().clone();

    let res: Vec<MovieRow> = reader
        .records()
       // .take(5)
        .flat_map(|x| x)
        .flat_map(|s| from_record(&s, &headers))
        .flat_map(|x| x.validated_movie_row(&config.last_run))
        .collect();

    for row in res {
        println!("row: {:?}", row)
    }
}

struct Config {
    input_file: String,
    last_run: Option<NaiveDate>,
}

fn parse_args(args: &mut impl Iterator<Item = String>) -> Config {
    args.next();

    let input_file = args.next().expect("msg");

    let last_run = args.next().map(|s| {
        NaiveDate::parse_from_str(s.as_ref(), "%Y-%m").expect("invalid last run; expected YYYY-MM")
    });

    Config {
        input_file,
        last_run,
    }
}
