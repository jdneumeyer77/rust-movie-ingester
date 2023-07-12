#[cfg(test)]
mod query_tests {
    use super::super::*;
    use chrono::NaiveDate;
    use std::{collections::{BTreeMap, HashMap}, rc::Rc};

    #[derive(Debug, Clone, PartialEq)]
    struct Dummy {
        id: i64,
        date: NaiveDate,
    }

    impl Dummy {
      fn new(id: i64, date: NaiveDate) -> Rc<Dummy> { Rc::new(Dummy {
            id,
            date,
        })
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
        assert_eq!(
            map.get(&2021).unwrap()[4].get(&1).unwrap().id,
            detail1.id
        );

        // Add details for the year 2022, month 7
        let detail2 = Dummy::new(2, NaiveDate::from_ymd(2022, 7, 1));
        add_detail(&mut map, &detail2);
        assert_eq!(map.len(), 2);
        assert_eq!(map.get(&2022).unwrap().len(), 12);
        assert_eq!(
            map.get(&2022).unwrap()[6].get(&2).unwrap().id,
            detail2.id
        );

        // Add details for an existing year and month
        let detail3 = Dummy::new(
            3,
            NaiveDate::from_ymd(2022, 7, 1)
        );
        add_detail(&mut map, &detail3);
        assert_eq!(map.len(), 2);
        assert_eq!(map.get(&2022).unwrap().len(), 12);
        assert_eq!(
            map.get(&2022).unwrap()[6].get(&3).unwrap().id,
            detail3.id
        );
    }

    #[test]
    fn test_flatten_bucket_year_map() {
        let mut map: BucketYearMap<Dummy> = BTreeMap::new();

        // Add details for the year 2021, month 5
        let detail1 = Dummy::new(
            1,
            NaiveDate::from_ymd(2021, 5, 1),
        );
        add_detail(&mut map, &detail1);

        // Add details for the year 2022, month 7
        let detail2 = Dummy::new(
            2,
            NaiveDate::from_ymd(2022, 7, 1),
        );
        add_detail(&mut map, &detail2);

        let flattened_map = flatten_bucket_year_map(&map);
        assert_eq!(flattened_map.len(), 2);
        assert_eq!(flattened_map.get(&2021).unwrap().len(), 1);
        assert_eq!(flattened_map.get(&2022).unwrap().len(), 1);
        assert_eq!(
            flattened_map.get(&2021).unwrap()[0][0].id,
            detail1.id
        );
        assert_eq!(
            flattened_map.get(&2022).unwrap()[0][0].id,
            detail2.id
        );
    }

    #[test]
    fn test_add_detail_months() {
        let mut map: BucketYearMap<Dummy> = BTreeMap::new();

        // Add details for the year 2021, month 5
        let detail1 = Dummy::new(
            1,
            NaiveDate::from_ymd(2021, 5, 1),
        );
        add_detail(&mut map, &detail1);
        assert_eq!(map.len(), 1);
        assert_eq!(map.get(&2021).unwrap().len(), 12);
        assert_eq!(
            map.get(&2021).unwrap()[4].get(&1).unwrap().id,
            detail1.id
        );

        // Add details for the year 2022, month 7
        let detail2 = Dummy::new(
            2,
            NaiveDate::from_ymd(2022, 7, 1),
        );
        add_detail(&mut map, &detail2);
        assert_eq!(map.len(), 2);
        assert_eq!(map.get(&2022).unwrap().len(), 12);
        assert_eq!(
            map.get(&2022).unwrap()[6].get(&2).unwrap().id,
            detail2.id
        );

        // Add details for an existing year and month
        let detail3 = Dummy::new(
            3,
            NaiveDate::from_ymd(2022, 7, 1),
        );
        add_detail(&mut map, &detail3);
        assert_eq!(map.len(), 2);
        assert_eq!(map.get(&2022).unwrap().len(), 12);
        assert_eq!(
            map.get(&2022).unwrap()[6].get(&3).unwrap().id,
            detail3.id
        );

        // Add details for a different month of an existing year
        let detail4 = Dummy::new(
            4,
            NaiveDate::from_ymd(2022, 8, 1),
        );
        add_detail(&mut map, &detail4);
        assert_eq!(map.len(), 2);
        assert_eq!(map.get(&2022).unwrap().len(), 12);
        assert_eq!(
            map.get(&2022).unwrap()[7].get(&4).unwrap().id,
            detail4.id
        );
    }

        #[test]
    fn test_flatten_bucket_year_map2() {
        let mut map: BucketYearMap<Dummy> = BTreeMap::new();

        // Add details for the year 2021, month 5
        let detail1 = Dummy::new(
            1,
            NaiveDate::from_ymd(2021, 5, 1),
        );
        add_detail(&mut map, &detail1);

        // Add details for the year 2022, month 7
        let detail2 = Dummy::new(
            2,
            NaiveDate::from_ymd(2022, 7, 1),
        );
        add_detail(&mut map, &detail2);

        let flattened_map = flatten_bucket_year_map(&map);
        assert_eq!(flattened_map.len(), 2);
        assert_eq!(flattened_map.get(&2021).unwrap().len(), 1);
        assert_eq!(flattened_map.get(&2022).unwrap().len(), 1);
        assert_eq!(
            flattened_map.get(&2021).unwrap()[0][0].id,
            detail1.id
        );
        assert_eq!(
            flattened_map.get(&2022).unwrap()[0][0].id,
            detail2.id
        );

        // Add details for an additional month in the year 2022
        let detail3 = Dummy::new(
            3,
            NaiveDate::from_ymd(2022, 8, 1),
        );
        add_detail(&mut map, &detail3);

        let flattened_map = flatten_bucket_year_map(&map);
        assert_eq!(flattened_map.len(), 2);
        assert_eq!(flattened_map.get(&2021).unwrap().len(), 1);
        assert_eq!(flattened_map.get(&2022).unwrap().len(), 2);
        assert_eq!(
            flattened_map.get(&2021).unwrap()[0][0].id,
            detail1.id
        );
        assert_eq!(
            flattened_map.get(&2022).unwrap()[0][0].id,
            detail2.id
        );
        assert_eq!(
            flattened_map.get(&2022).unwrap()[1][0].id,
            detail3.id
        );

        // Add details for a different year
        let detail4 = Dummy::new(
            4,
            NaiveDate::from_ymd(2023, 1, 1),
        );
        add_detail(&mut map, &detail4);

        let flattened_map = flatten_bucket_year_map(&map);
        assert_eq!(flattened_map.len(), 3);
        assert_eq!(flattened_map.get(&2021).unwrap().len(), 1);
        assert_eq!(flattened_map.get(&2022).unwrap().len(), 2);
        assert_eq!(flattened_map.get(&2023).unwrap().len(), 1);
        assert_eq!(
            flattened_map.get(&2021).unwrap()[0][0].id,
            detail1.id
        );
        assert_eq!(
            flattened_map.get(&2022).unwrap()[0][0].id,
            detail2.id
        );
        assert_eq!(
            flattened_map.get(&2022).unwrap()[1][0].id,
            detail3.id
        );
        assert_eq!(
            flattened_map.get(&2023).unwrap()[0][0].id,
            detail4.id
        );
    }

    #[test]
    fn test_flatten_bucket_year_map3() {
        let mut map: BucketYearMap<Dummy> = BTreeMap::new();

        // Add details for the year 2021, month 5
        let detail1 = Dummy::new(
            1,
            NaiveDate::from_ymd(2021, 5, 1),
        );
        add_detail(&mut map, &detail1);

        // Add details for the year 2022, month 7
        let detail2 = Dummy::new(
            2,
            NaiveDate::from_ymd(2022, 7, 1),
        );
        add_detail(&mut map, &detail2);

        let flattened_map = flatten_bucket_year_map(&map);
        assert_eq!(flattened_map.len(), 2);
        assert_eq!(flattened_map.get(&2021).unwrap().len(), 1);
        assert_eq!(flattened_map.get(&2022).unwrap().len(), 1);
        assert_eq!(
            flattened_map.get(&2021).unwrap()[0][0].id,
            detail1.id
        );
        assert_eq!(
            flattened_map.get(&2022).unwrap()[0][0].id,
            detail2.id
        );

        // Add details for an additional month in the year 2022
        let detail3 = Dummy::new(
            3,
            NaiveDate::from_ymd(2022, 8, 1),
        );
        add_detail(&mut map, &detail3);

        let flattened_map = flatten_bucket_year_map(&map);
        assert_eq!(flattened_map.len(), 2);
        assert_eq!(flattened_map.get(&2021).unwrap().len(), 1);
        assert_eq!(flattened_map.get(&2022).unwrap().len(), 2);
        assert_eq!(
            flattened_map.get(&2021).unwrap()[0][0].id,
            detail1.id
        );
        assert_eq!(
            flattened_map.get(&2022).unwrap()[0][0].id,
            detail2.id
        );
        assert_eq!(
            flattened_map.get(&2022).unwrap()[1][0].id,
            detail3.id
        );

        // Add details for a different year
        let detail4 = Dummy::new(
            4,
            NaiveDate::from_ymd(2023, 1, 1),
        );
        add_detail(&mut map, &detail4);

        let flattened_map = flatten_bucket_year_map(&map);
        assert_eq!(flattened_map.len(), 3);
        assert_eq!(flattened_map.get(&2021).unwrap().len(), 1);
        assert_eq!(flattened_map.get(&2022).unwrap().len(), 2);
        assert_eq!(flattened_map.get(&2023).unwrap().len(), 1);
        assert_eq!(
            flattened_map.get(&2021).unwrap()[0][0].id,
            detail1.id
        );
        assert_eq!(
            flattened_map.get(&2022).unwrap()[0][0].id,
            detail2.id
        );
        assert_eq!(
            flattened_map.get(&2022).unwrap()[1][0].id,
            detail3.id
        );
        assert_eq!(
            flattened_map.get(&2023).unwrap()[0][0].id,
            detail4.id
        );

        // Add details for the same year and month but different IDs
        let detail5 = Dummy::new(
            5,
            NaiveDate::from_ymd(2022, 8, 1),
        );
        add_detail(&mut map, &detail5);

        let flattened_map = flatten_bucket_year_map(&map);
        assert_eq!(flattened_map.len(), 3);
        assert_eq!(flattened_map.get(&2021).unwrap().len(), 1);
        assert_eq!(flattened_map.get(&2022).unwrap().len(), 2);
        assert_eq!(flattened_map.get(&2023).unwrap().len(), 1);
        assert_eq!(
            flattened_map.get(&2021).unwrap()[0][0].id,
            detail1.id
        );
        assert_eq!(
            flattened_map.get(&2022).unwrap()[0][0].id,
            detail2.id
        );
        assert_eq!(
            flattened_map.get(&2022).unwrap()[1].len(),
            2
        );
        assert!(
            flattened_map.get(&2022).unwrap()[1]
                .iter()
                .any(|d| d.id == detail3.id)
        );
        assert!(
            flattened_map.get(&2022).unwrap()[1]
                .iter()
                .any(|d| d.id == detail5.id)
        );
        assert_eq!(
            flattened_map.get(&2023).unwrap()[0][0].id,
            detail4.id
        );
    }
}