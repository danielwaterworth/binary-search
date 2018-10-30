# binary-search

Given a monotone function, find the largest quantity that is too small
and the smallest quantity that is too large. The first two arguments to
`binary_search` set the bounds of the search space.

    use binary_search::{binary_search, Direction};

    fn main() {
      let values =
        [0, 4, 5, 6, 7, 9, 456];

      let (largest_low, smallest_high) =
        binary_search((0, ()), (values.len(), ()), |i|
          if values[i] < 6 {
            Direction::Low(())
          } else {
            Direction::High(())
          }
        );

      dbg!(largest_low);
      dbg!(smallest_high);
    }

You can also provide an associated 'witness' as in this
example. Witnesses are passed in as well as produced from
`binary_search`. The arguments act as a proof that the function does
indeed transition within the range. If you don't know that this is the
case, you may need to call your function at the bounds first.

    use binary_search::{binary_search, Direction};

    fn main() {
      let values =
        [Ok("foo"), Ok("bar"), Ok("baz"), Err(false), Err(true)];

      let (largest_low, smallest_high) =
        binary_search((0, "foo"), (values.len() - 1, true), |i|
          match values[i] {
            Ok(x) => Direction::Low(x),
            Err(x) => Direction::High(x),
          }
        );

      dbg!(largest_low); // "baz"
      dbg!(smallest_high); // false
    }
