#![deny(clippy::all)]

use napi_derive::napi;

#[napi]
pub fn plus(a: u32, b: u32) -> u32 {
  a + b
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_plus() {
    assert_eq!(plus(2, 3), 5);
  }
}
