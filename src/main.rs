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

    let res: Vec<ProdCompanyDetails> = res
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
        NaiveDate::parse_from_str(s.as_ref(), "%Y-%m").expect("invalid last run; expected YYYY-MM")
    });

    Config {
        input_file,
        last_run,
    }
}

mod query {
    use chrono::{Datelike, NaiveDate};
    use itertools::Itertools;
    use std::{collections::{BTreeMap, HashMap}, rc::Rc};

    // use self::by_production_companies::prod_company_details;

    pub trait ById {
        fn id(&self) -> i64;
        fn date(&self) -> &NaiveDate;
        fn sum(&self, other: &Self) -> Rc<Self>;
        // fn zero(&self) -> Box<Self>;
    }

    pub type BucketYearMap<T> = BTreeMap<i32, [HashMap<i64, Rc<T>>; 12]>;
    pub type BucketYearMapFlattned<T> = BTreeMap<i32, Vec<Vec<Rc<T>>>>;

    pub fn add_detail<'a, T: ById + Clone>(map: &'a mut BucketYearMap<T>, detail: &T) -> &'a mut BucketYearMap<T> {
        let year = detail.date().year();
        let month: usize = (detail.date().month() - 1).try_into().unwrap_or(0);
        map.entry(year)
            .and_modify(|x| {
                x[month]
                    .entry(detail.id())
                    .and_modify(|x| { *x =  x.sum(detail); })
                    .or_insert(Rc::new(detail.clone()));
            })
            .or_insert(Default::default());

        map
    }

    pub fn flatten_bucket_year_map<T>(
        map: &BucketYearMap<T>,
    ) -> BucketYearMapFlattned<T> {
        map.iter()
            .filter(|(year, months)| !months.iter().all(|x| x.is_empty()))
            .map(|(year, months)| {
                let flatten = months
                .iter()
                .filter(|x| !x.is_empty())
                .map(|m| m.iter().map(|x| x.1.clone()).collect_vec())
                .collect_vec();

                (*year, flatten)
            })
            .collect()
    }

    pub mod by_production_companies {
        use std::{collections::HashSet, rc::Rc};

        use chrono::NaiveDate;
        use kstring::KString;

        use crate::data::Movie;

        #[derive(Debug, Clone)]
        pub struct ProdCompanyMetadata {
            movieIds: HashSet<KString>,
            genreIds: HashSet<i64>,
        }
        #[derive(Debug, Clone)]
        pub struct ProdCompanyDetails {
            id: i64,
            date: NaiveDate, // does this make sense...
            budget: i64,
            profit: i64,
            revenue: i64,
            avg_populatarity: f32,
            metadata: ProdCompanyMetadata,
        }

        // impl From<&Movie> for Vec<ProdCompanyDetails> {
        pub fn movie_to_details(value: &Movie) -> Vec<ProdCompanyDetails> {
            value
                .production_companies
                .iter()
                .map(|prod| ProdCompanyDetails {
                    id: *prod,
                    date: value.release_date,
                    budget: value.budget,
                    profit: value.profit,
                    revenue: value.revenue,
                    avg_populatarity: value.avg_populatarity,
                    metadata: ProdCompanyMetadata {
                        movieIds: HashSet::from([value.id.clone(); 1]),
                        genreIds: value.genres.clone(),
                    },
                })
                .collect()
        }
        // }

        impl super::ById for ProdCompanyDetails {
            fn id(&self) -> i64 {
                self.id
            }

            fn date(&self) -> &NaiveDate {
                &self.date
            }

            fn sum(&self, other: &Self) -> Rc<Self> {
                let details = ProdCompanyDetails {
                    id: self.id,
                    date: self.date.clone(),
                    budget: self.budget + other.budget,
                    profit: self.profit + other.profit,
                    revenue: self.revenue + other.revenue,
                    avg_populatarity: (self.avg_populatarity + other.avg_populatarity) / 2.0,
                    // probably not the best performance... but immutable.
                    metadata: ProdCompanyMetadata {
                        movieIds: self
                            .metadata
                            .movieIds
                            .union(&other.metadata.movieIds)
                            .cloned()
                            .collect(),
                        genreIds: self
                            .metadata
                            .genreIds
                            .union(&other.metadata.genreIds)
                            .copied()
                            .collect(),
                    },
                };

                Rc::new(details)
            }
        }
    }
}
