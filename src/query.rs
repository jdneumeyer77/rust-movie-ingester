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

fn upsert_details<T: ById + Clone>(map: &mut HashMap<i64, Rc<T>>, detail: &Rc<T>) {
  map.entry(detail.id())
      .and_modify(|x| { *x =  x.sum(&detail); })
      .or_insert(detail.clone());
}

pub fn add_detail<'a, T: ById + Clone>(map: &'a mut BucketYearMap<T>, detail: &Rc<T>) -> &'a mut BucketYearMap<T> {
    let year = detail.date().year();
    let month: usize = (detail.date().month() - 1).try_into().unwrap_or(0);
    map.entry(year)
        .and_modify(|x| {
           upsert_details(&mut x[month], detail);
        })
        .or_insert_with(|| {
          let mut default: [HashMap<i64, Rc<T>>; 12] = Default::default();
          upsert_details(&mut default[month], detail);
          default
        });

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

pub mod by_production_companies;
mod query_test;
