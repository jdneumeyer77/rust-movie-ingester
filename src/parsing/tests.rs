
#[cfg(test)]
mod tests {
  use std::collections::hash_set;

use super::super::*;

  #[test]
  fn json_to_set() {
    let json = "[{'id': 28, 'name': 'Action'}, {'id': 12, 'name': 'Adventure'}, {'id': 878, 'name': 'Science Fiction'}, {'id': 53, 'name': 'Thriller'}]";

    assert_eq!(convert_json_to_set(""), HashSet::new());
    let expected = HashSet::from(["28","12","878", "53"].map(|x| x.to_string()));
    assert_eq!(convert_json_to_set(json), expected);
  }
}