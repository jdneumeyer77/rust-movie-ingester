use chrono::NaiveDate;
use fs::File;
use std::{
    borrow::BorrowMut,
    collections::{BTreeMap, HashSet},
    env, fs,
};

use crate::{data::*, query::by_production_companies::*, query::*};

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

    let mut acc: BucketYearMap<ProdCompanyDetails> = BTreeMap::new();

    let res: Vec<_> = res
        .iter()
        .flat_map(|movie| movie_to_details(movie))
        .collect();

// TODO: figure how to get fold + &mut to work.
    // let x: &BucketYearMap<ProdCompanyDetails> = res
    //     .iter()
    //     .fold(acc.borrow_mut(), |mut acc, detail| add_detail(acc, detail));

    // let barrowed_acc = acc.borrow_mut();
    //let _ = res.iter().for_each(|detail| { add_detail(acc, detail); } );

    
    for detail in &res {
        add_detail(&mut acc, detail);
    }
    
    //let flat = flatten_bucket_year_map(&acc);

    // let res: Vec<_> = acc
    //     .iter()
    //     .filter(|(year, months)| !months.iter().all(|x| x.is_empty()))
    //   //  .take(5)
    //     .collect();

    let flattened = flatten_bucket_year_map(&acc);

    flattened.iter().take(5).for_each(|(year, v)| {
        println!("year {year}:");
        let remove_empty: Vec<_> = v.iter().filter(|x| !x.is_empty()).collect();
        println!("{:?}", remove_empty)
    });

    //.fold(&mut acc, |&mut acc, next| add_detail(acc, &next));

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
        println!("last run: {s}");
        let expand_date: String = format!("{s}-01");
        NaiveDate::parse_from_str(&expand_date, "%Y-%m-%d").expect("invalid last run; expected YYYY-MM")
    });

    Config {
        input_file,
        last_run,
    }
}

mod query;

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::*;

    #[test]
    fn test_parse_args() {
        // Test case 1: Valid input arguments
        let args = &mut vec![
            "program_name".to_string(),
            "input.txt".to_string(),
            "2021-07".to_string(),
        ]
        .into_iter();
        let config = parse_args(args);
        assert_eq!(config.input_file, "input.txt");
        assert_eq!(config.last_run.unwrap().year(), 2021);
        assert_eq!(config.last_run.unwrap().month(), 7);

        // Test case 2: Missing input file argument
        // let args = &mut vec!["program_name".to_string()].into_iter();
        // let config = parse_args(args);
        // assert_eq!(config.input_file, ""); // Assuming empty string is the default value for input_file
        // assert_eq!(config.last_run, None);

        // Test case 3: Invalid last run argument format
        // let args = &mut vec![
        //     "program_name".to_string(),
        //     "input.txt".to_string(),
        //     "2021-13".to_string(),
        // ]
        // .into_iter();
        // let result = std::panic::catch_unwind(|| parse_args(args));
        // assert!(result.is_err());
    }
}

