#[cfg(test)]
mod query_tests {
    use super::super::*;
    use chrono::NaiveDate;
    use rand::{rngs::ThreadRng, *};
    use std::{
        collections::{BTreeMap, HashMap},
        hash::Hash,
        rc::Rc,
    };
    #[derive(Debug, Clone)]
    struct Dummy {
        id: i64,
        date: NaiveDate,
        profit: i64,
    }

    impl PartialEq for Dummy {
        fn eq(&self, other: &Self) -> bool {
            self.id == other.id && self.date == other.date
        }
    }

    impl Dummy {
        fn new(id: i64, date: NaiveDate) -> Rc<Dummy> {
            let profit = ThreadRng::default().gen_range(0..=9999i64);
            Rc::new(Dummy { id, date, profit })
        }
    }

    impl ById for Dummy {
        fn id(&self) -> i64 {
            self.id
        }

        fn date(&self) -> &NaiveDate {
            &self.date
        }

        fn sum(&self, other: &Self) -> Rc<Self> {
            Rc::new(Dummy {
                id: self.id,
                date: self.date.clone(),
                profit: self.profit + other.profit,
            })
        }
    }

    #[test]
    fn test_add_detail() {
        let mut map: BucketYearMap<Dummy> = BTreeMap::new();
        assert_eq!(map.len(), 0);
        // Add details for the year 2021, month 5
        let detail1 = Dummy::new(1, NaiveDate::from_ymd(2021, 5, 1));
        add_detail(&mut map, &detail1);
        assert_eq!(map.len(), 1);
        assert_eq!(map.get(&2022), None);
        assert_eq!(map.get(&2021).unwrap().len(), 12);
        assert_eq!(map.get(&2021).unwrap()[4].get(&1).unwrap().id, detail1.id);

        // Add details for the year 2022, month 7
        let detail2 = Dummy::new(2, NaiveDate::from_ymd(2022, 7, 1));
        add_detail(&mut map, &detail2);
        assert_eq!(map.len(), 2);
        assert_eq!(map.get(&2022).unwrap().len(), 12);
        assert_eq!(map.get(&2022).unwrap()[6].get(&2).unwrap().id, detail2.id);

        // Add details for an existing year and month
        let detail3 = Dummy::new(3, NaiveDate::from_ymd(2022, 7, 1));
        add_detail(&mut map, &detail3);
        assert_eq!(map.len(), 2);
        assert_eq!(map.get(&2022).unwrap().len(), 12);
        assert_eq!(map.get(&2022).unwrap()[6].get(&3).unwrap().id, detail3.id);
    }

    #[test]
    fn sum_add_details() {
        let mut map: BucketYearMap<Dummy> = BTreeMap::new();

        let detail1 = Dummy::new(1, NaiveDate::from_ymd(2021, 5, 1));
        let detail2 = Dummy::new(1, NaiveDate::from_ymd(2021, 5, 4));
        let detail4 = Dummy::new(2, NaiveDate::from_ymd(2021, 5, 1));
        let detail3 = Dummy::new(1, NaiveDate::from_ymd(2021, 4, 1));
        add_detail(&mut map, &detail1);
        add_detail(&mut map, &detail2);
        add_detail(&mut map, &detail3);
        add_detail(&mut map, &detail4);

        assert_eq!(map.get(&2021).unwrap()[4].len(), 2);
        assert_eq!(
            map.get(&2021).unwrap()[4].get(&1).unwrap().profit,
            detail1.profit + detail2.profit
        );
        assert_eq!(
            map.get(&2021).unwrap()[3].get(&1).unwrap().profit,
            detail3.profit
        );

        let flattened = flatten_bucket_year_map(&map);
        println!("{:?}", flattened);

        let flattened_group = flattened.get(&2021).unwrap().iter().group_by(|x| x.id);
        let flattened_group: HashMap<_, _> = flattened_group
            .into_iter()
            .map(|(k, v)| (k, v.collect_vec().len()))
            .collect();
        println!("{:?}", flattened_group);
        assert_eq!(flattened.get(&2021).unwrap().len(), 3);
        assert_eq!(flattened_group.get(&1).unwrap(), &2);
        assert_eq!(flattened_group.get(&2).unwrap(), &1);
    }

    #[test]
    fn test_flatten_bucket_year_map() {
        let mut map: BucketYearMap<Dummy> = BTreeMap::new();

        // Add details for the year 2021, month 5
        let detail1 = Dummy::new(1, NaiveDate::from_ymd(2021, 5, 1));
        add_detail(&mut map, &detail1);

        // Add details for the year 2022, month 7
        let detail2 = Dummy::new(2, NaiveDate::from_ymd(2022, 7, 1));
        add_detail(&mut map, &detail2);

        let flattened_map = flatten_bucket_year_map(&map);
        assert_eq!(flattened_map.len(), 2);
        assert_eq!(flattened_map.get(&2021).unwrap().len(), 1);
        assert_eq!(flattened_map.get(&2022).unwrap().len(), 1);
        assert_eq!(flattened_map.get(&2021).unwrap()[0].id, detail1.id);
        assert_eq!(flattened_map.get(&2022).unwrap()[0].id, detail2.id);
    }

    #[test]
    fn test_add_detail_months() {
        let mut map: BucketYearMap<Dummy> = BTreeMap::new();

        // Add details for the year 2021, month 5
        let detail1 = Dummy::new(1, NaiveDate::from_ymd(2021, 5, 1));
        add_detail(&mut map, &detail1);
        assert_eq!(map.len(), 1);
        assert_eq!(map.get(&2021).unwrap().len(), 12);
        assert_eq!(map.get(&2021).unwrap()[4].get(&1).unwrap().id, detail1.id);

        // Add details for the year 2022, month 7
        let detail2 = Dummy::new(2, NaiveDate::from_ymd(2022, 7, 1));
        add_detail(&mut map, &detail2);
        assert_eq!(map.len(), 2);
        assert_eq!(map.get(&2022).unwrap().len(), 12);
        assert_eq!(map.get(&2022).unwrap()[6].get(&2).unwrap().id, detail2.id);

        // Add details for an existing year and month
        let detail3 = Dummy::new(3, NaiveDate::from_ymd(2022, 7, 1));
        add_detail(&mut map, &detail3);
        assert_eq!(map.len(), 2);
        assert_eq!(map.get(&2022).unwrap().len(), 12);
        assert_eq!(map.get(&2022).unwrap()[6].get(&3).unwrap().id, detail3.id);

        // Add details for a different month of an existing year
        let detail4 = Dummy::new(4, NaiveDate::from_ymd(2022, 8, 1));
        add_detail(&mut map, &detail4);
        assert_eq!(map.len(), 2);
        assert_eq!(map.get(&2022).unwrap().len(), 12);
        assert_eq!(map.get(&2022).unwrap()[7].get(&4).unwrap().id, detail4.id);
    }

    #[test]
    fn test_flatten_bucket_year_map2() {
        let mut map: BucketYearMap<Dummy> = BTreeMap::new();

        // Add details for the year 2021, month 5
        let detail1 = Dummy::new(1, NaiveDate::from_ymd(2021, 5, 1));
        add_detail(&mut map, &detail1);

        // Add details for the year 2022, month 7
        let detail2 = Dummy::new(2, NaiveDate::from_ymd(2022, 7, 1));
        add_detail(&mut map, &detail2);

        let flattened_map = flatten_bucket_year_map(&map);
        assert_eq!(flattened_map.len(), 2);
        assert_eq!(flattened_map.get(&2021).unwrap().len(), 1);
        assert_eq!(flattened_map.get(&2022).unwrap().len(), 1);
        assert_eq!(flattened_map.get(&2021).unwrap()[0].id, detail1.id);
        assert_eq!(flattened_map.get(&2022).unwrap()[0].id, detail2.id);

        // Add details for an additional month in the year 2022
        let detail3 = Dummy::new(3, NaiveDate::from_ymd(2022, 8, 1));
        add_detail(&mut map, &detail3);

        let flattened_map = flatten_bucket_year_map(&map);
        assert_eq!(flattened_map.len(), 2);
        assert_eq!(flattened_map.get(&2021).unwrap().len(), 1);
        assert_eq!(flattened_map.get(&2022).unwrap().len(), 2);
        assert_eq!(flattened_map.get(&2021).unwrap()[0].id, detail1.id);
        assert_eq!(flattened_map.get(&2022).unwrap()[0].id, detail2.id);
        assert_eq!(flattened_map.get(&2022).unwrap()[1].id, detail3.id);

        // Add details for a different year
        let detail4 = Dummy::new(4, NaiveDate::from_ymd(2023, 1, 1));
        add_detail(&mut map, &detail4);

        let flattened_map = flatten_bucket_year_map(&map);
        assert_eq!(flattened_map.len(), 3);
        assert_eq!(flattened_map.get(&2021).unwrap().len(), 1);
        assert_eq!(flattened_map.get(&2022).unwrap().len(), 2);
        assert_eq!(flattened_map.get(&2023).unwrap().len(), 1);
        assert_eq!(flattened_map.get(&2021).unwrap()[0].id, detail1.id);
        assert_eq!(flattened_map.get(&2022).unwrap()[0].id, detail2.id);
        assert_eq!(flattened_map.get(&2022).unwrap()[1].id, detail3.id);
        assert_eq!(flattened_map.get(&2023).unwrap()[0].id, detail4.id);
    }

    #[test]
    fn test_flatten_bucket_year_map3() {
        let mut map: BucketYearMap<Dummy> = BTreeMap::new();

        // Add details for the year 2021, month 5
        let detail1 = Dummy::new(1, NaiveDate::from_ymd(2021, 5, 1));
        add_detail(&mut map, &detail1);

        // Add details for the year 2022, month 7
        let detail2 = Dummy::new(2, NaiveDate::from_ymd(2022, 7, 1));
        add_detail(&mut map, &detail2);

        let flattened_map = flatten_bucket_year_map(&map);
        assert_eq!(flattened_map.len(), 2);
        assert_eq!(flattened_map.get(&2021).unwrap().len(), 1);
        assert_eq!(flattened_map.get(&2022).unwrap().len(), 1);
        assert_eq!(flattened_map.get(&2021).unwrap()[0].id, detail1.id);
        assert_eq!(flattened_map.get(&2022).unwrap()[0].id, detail2.id);

        // Add details for an additional month in the year 2022
        let detail3 = Dummy::new(3, NaiveDate::from_ymd(2022, 8, 1));
        add_detail(&mut map, &detail3);

        let flattened_map = flatten_bucket_year_map(&map);
        assert_eq!(flattened_map.len(), 2);
        assert_eq!(flattened_map.get(&2021).unwrap().len(), 1);
        assert_eq!(flattened_map.get(&2022).unwrap().len(), 2);
        assert_eq!(flattened_map.get(&2021).unwrap()[0].id, detail1.id);
        assert_eq!(flattened_map.get(&2022).unwrap()[0].id, detail2.id);
        assert_eq!(flattened_map.get(&2022).unwrap()[1].id, detail3.id);

        // Add details for a different year
        let detail4 = Dummy::new(4, NaiveDate::from_ymd(2023, 1, 1));
        add_detail(&mut map, &detail4);

        let flattened_map = flatten_bucket_year_map(&map);
        assert_eq!(flattened_map.len(), 3);
        assert_eq!(flattened_map.get(&2021).unwrap().len(), 1);
        assert_eq!(flattened_map.get(&2022).unwrap().len(), 2);
        assert_eq!(flattened_map.get(&2023).unwrap().len(), 1);
        assert_eq!(flattened_map.get(&2021).unwrap()[0].id, detail1.id);
        assert_eq!(flattened_map.get(&2022).unwrap()[0].id, detail2.id);
        assert_eq!(flattened_map.get(&2022).unwrap()[1].id, detail3.id);
        assert_eq!(flattened_map.get(&2023).unwrap()[0].id, detail4.id);

        // Add details for the same year and month but different IDs
        let detail5 = Dummy::new(5, NaiveDate::from_ymd(2022, 8, 1));
        add_detail(&mut map, &detail5);

        // Add details for the same year and month but same IDs
        let detail6 = Dummy::new(3, NaiveDate::from_ymd(2022, 8, 1));
        add_detail(&mut map, &detail6);

        let flattened_map = flatten_bucket_year_map(&map);
        assert_eq!(flattened_map.len(), 3);
        assert_eq!(flattened_map.get(&2021).unwrap().len(), 1);
        assert_eq!(flattened_map.get(&2022).unwrap().len(), 3);
        assert_eq!(flattened_map.get(&2023).unwrap().len(), 1);
        assert_eq!(flattened_map.get(&2021).unwrap()[0].id, detail1.id);
        assert_eq!(flattened_map.get(&2022).unwrap()[0].id, detail2.id);
        assert!(flattened_map
            .get(&2022)
            .unwrap()
            .iter()
            .skip(1)
            .any(|d| d.id == detail3.id));
        assert!(flattened_map
            .get(&2022)
            .unwrap()
            .iter()
            .skip(1)
            .any(|d| d.id == detail5.id));
        assert_eq!(flattened_map.get(&2023).unwrap()[0].id, detail4.id);
    }
}
