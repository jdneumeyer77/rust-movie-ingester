use chrono::NaiveDate;
use fs::File;
use std::{collections::HashSet, env, fs, io::BufReader};

// TODO: reorganize.

pub mod parsing;

#[derive(Debug)]
pub struct MovieRow {
    id: String,
    genres: HashSet<String>,
    production_companies: HashSet<String>,
    release_date: NaiveDate,
    budget: i128,
    revenue: i128,
    profit: i128,
    avg_populatarity: f32,
    status: Status,
}

#[derive(PartialEq, Debug, Hash, Eq)]
enum Status {
    Released,
    Other,
}

impl Status {
    fn from_str(enum_str: &str) -> Status {
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
        .flat_map(|s| parsing::from_record(&s, &headers))
        .flat_map(|x| x.validated_movie_row(&config.last_run))
        .collect();

    let distinct: HashSet<&Status> = res.iter().map(|x| &x.status).collect();

    for row in distinct {
        println!("{:?}", row)
    }

    // for row in res {
    //     println!("row: {:?}", row)
    // }
}

struct Config {
    input_file: String,
    last_run: Option<NaiveDate>,
}

// TODO: use clap? Probably overkill.
fn parse_args(args: &mut impl Iterator<Item = String>) -> Config {
    args.next();

    let input_file = args.next().expect("missing input file!");

    let last_run = args.next().map(|s| {
        NaiveDate::parse_from_str(s.as_ref(), "%Y-%m").expect("invalid last run; expected YYYY-MM")
    });

    Config {
        input_file,
        last_run,
    }
}
