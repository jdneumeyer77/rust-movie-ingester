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
pub fn movie_to_details(value: &Movie) -> Vec<Rc<ProdCompanyDetails>> {
    value
        .production_companies
        .iter()
        .map(|prod| {
            Rc::new(ProdCompanyDetails {
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
