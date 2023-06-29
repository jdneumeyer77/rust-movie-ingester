#[cfg(test)]
mod tests {
    use std::collections::hash_set;

    use super::super::*;

    #[test]
    fn json_to_set() {
        assert_eq!(convert_json_to_set(""), HashSet::new());
        assert_eq!(convert_json_to_set("[]"), HashSet::new());
        let invalid_json = r#"[{"a": 2}, {"a": 3}]"#;
        assert_eq!(convert_json_to_set(invalid_json), HashSet::new());
        let json_single_quotes = "[{'id': 28, 'name': 'Action'}, {'id': 12, 'name': 'Adventure'}, {'id': 878, 'name': 'Science Fiction'}, {'id': 53, 'name': 'Thriller'}]";
        let json = json_single_quotes.replace("'", "\"");
        let expected = HashSet::from([28, 12, 878, 53]);
        assert_eq!(convert_json_to_set(&json), expected);
        assert_eq!(convert_json_to_set(&json_single_quotes), expected);
    }
}
