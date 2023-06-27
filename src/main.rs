use chrono::NaiveDate;
use fs::File;
use std::{collections::HashSet, env, fs};

use crate::data::*;

// TODO: reorganize.

pub mod data;
pub mod parsing;

fn main() {
    let args = env::args();
    let config = parse_args(args.into_iter().by_ref());

    let file = File::open(&config.input_file).expect("Couldn't read file...");
    println!("opened file for reading: {}", &config.input_file);
    let res: Vec<Movie> = read_movie_metadata(&file, &config);
    let distinct: HashSet<&Status> = res.iter().map(|x| &x.status).collect();

    for row in distinct {
        println!("{:?}", row)
    }

    // for row in res {
    //     println!("row: {:?}", row)
    // }
}

pub struct Config {
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
