
#[cfg(test)]
mod tests {
  use super::super::*;

  #[test]
  fn status_from_str_basics() {
    assert_eq!(Status::from_str("bob"), Status::Other);
    assert_eq!(Status::from_str(""), Status::Other);
    assert_eq!(Status::from_str("released"), Status::Released);
    assert_eq!(Status::from_str("Released"), Status::Released);
    assert_eq!(Status::from_str("RelEAsed"), Status::Released);
  }

}