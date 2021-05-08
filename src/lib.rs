use std::ops::{Shr, Add, BitAnd};

pub trait Betweenable where Self: Copy {
  fn between(x: Self, y: Self) -> Option<Self>;
}

impl<X> Betweenable for X
    where
      X: Copy,
      X: Shr<i8, Output=X>,
      X: Add<X, Output=X>,
      X: BitAnd<X, Output=X>,
      X: From<u8>,
      X: PartialOrd {
  fn between(low: Self, high: Self) -> Option<Self> {
    let one = X::from(1);
    if high <= low + one {
      None
    } else {
      let mid = (low >> 1) + (high >> 1) + (low & high & one);
      Some(mid)
    }
  }
}

pub enum Direction<A, B> {
  Low(A),
  High(B),
}

pub fn binary_search<X, A, B, F>(
    low: (X, A),
    high: (X, B),
    mut f: F,
  ) -> ((X, A), (X, B))
  where
    X: Betweenable,
    F: FnMut(X) -> Direction<A, B> {
  match X::between(low.0, high.0) {
    None => {
      (low, high)
    },
    Some(x) => {
      match (f)(x) {
        Direction::Low(low) => {
          binary_search((x, low), high, f)
        },
        Direction::High(high) => {
          binary_search(low, (x, high), f)
        },
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn split_usize() {
    assert_eq!(usize::between(1, 0), None);
    assert_eq!(usize::between(1, 1), None);
    assert_eq!(usize::between(1, 2), None);
    assert_eq!(usize::between(1, 3), Some(2));
    assert_eq!(
      usize::between(usize::max_value()-3, usize::max_value()-1),
      Some(usize::max_value()-2),
    );
    assert_eq!(
      usize::between(usize::max_value()-2, usize::max_value()),
      Some(usize::max_value()-1),
    );
  }

  #[test]
  fn binary_search_test() {
    let result =
      binary_search((1 as usize, ()), (100, ()), |x| {
        if x < 23 {
          Direction::Low(())
        } else {
          Direction::High(())
        }
      });
    assert_eq!(result, ((22, ()), (23, ())))
  }
}
