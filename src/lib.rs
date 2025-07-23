use std::ops::{Add, BitAnd, Shr};

pub trait Betweenable
where
    Self: Copy,
{
    fn between(x: Self, y: Self) -> Option<Self>;
}

impl<X> Betweenable for X
where
    X: Copy,
    X: Shr<i8, Output = X>,
    X: Add<X, Output = X>,
    X: BitAnd<X, Output = X>,
    X: From<u8>,
    X: PartialOrd,
{
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

///
/// Whether or not the current candidate transition point is lower or higher than the search target.
///
pub enum Direction<A, B> {
    Low(A),
    High(B),
}

///
/// Perform a binary search within the specified bounds.
///
/// Given a monotone function, find the largest quantity that is too small and the smallest quantity that is too large.
///
/// A user-specified function will be called to determine whether or not an entry is considered lower or higher.
///
/// ## Generics
///
/// - `X` - the type of the search space. used to specify the bounds of the search and is used for the returned `largest_low` and `smallest_high` values.
/// - `A` - the type for the low witness.
/// - `B` - the type for the high witness.
///
/// ## Arguments
/// ### `low`
/// A tuple specifying the lower bound of the search.
///
/// **Expanded as `(lower_bound, witness)` here for brevity.**
///
/// - `lower_bound: X` - the lower bound of the search space.\
/// - `witness: A` - a value passed to the provided function that can be used to verify whether or not a transition is valid.
///
/// ### `high`
/// A tuple specifying the upper bound of the search.
///
/// **Expanded as `(upper_bound, witness)` here for brevity.**
///
/// - `upper_bound: X` - the upper bound of the search space.\
/// - `witness: B` - a value passed to the provided function that can be used to verify whether or not a transition is valid.
///
/// ### `f`
/// A `FnMut(X)` function that is called to evaluate each candidate transition point.
///
/// #### Arguments
///
/// - `X` - the current candidate transition point.
///
/// #### Returns
///
/// - `Direction` - whether the current candidate transition point is lower or higher than the search target.
///
/// ## Return value
///
/// Returns a tuple `((largest_low: X, low_witness: A), (smallest_high: X, high_witness: B))`.
///
/// - `largest_low: X` - the largest value that the provided function indicated was `Low`.
/// - `low_witness: A` - the witness value that the provided function last returned inside the `Direction::Low`.
///
/// - `smallest_high: X` - the smallest value that the provided function indicated was `High`.
/// - `high_witness: B` - the witness value that the provided function last returned inside the `Direction::High`.
///
/// ## Examples
///
/// ```
/// use binary_search::{binary_search, Direction};
/// let result = binary_search((1 as usize, ()), (100, ()), |x| {
/// if x < 23 {
/// 	Direction::Low(())
/// } else {
/// 	Direction::High(())
/// }
/// });
/// assert_eq!(result, ((22, ()), (23, ())))
/// ```
///
pub fn binary_search<X, A, B, F>(low: (X, A), high: (X, B), mut f: F) -> ((X, A), (X, B))
where
    X: Betweenable,
    F: FnMut(X) -> Direction<A, B>,
{
    match X::between(low.0, high.0) {
        None => (low, high),
        Some(x) => match (f)(x) {
            Direction::Low(low) => binary_search((x, low), high, f),
            Direction::High(high) => binary_search(low, (x, high), f),
        },
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
            usize::between(usize::max_value() - 3, usize::max_value() - 1),
            Some(usize::max_value() - 2),
        );
        assert_eq!(
            usize::between(usize::max_value() - 2, usize::max_value()),
            Some(usize::max_value() - 1),
        );
    }

    #[test]
    fn binary_search_test() {
        let result = binary_search((1 as usize, ()), (100, ()), |x| {
            if x < 23 {
                Direction::Low(())
            } else {
                Direction::High(())
            }
        });
        assert_eq!(result, ((22, ()), (23, ())))
    }

    #[test]
    fn binary_search_simple_test() {
        let values = [0, 4, 5, 6, 7, 9, 456];

        let (largest_low, smallest_high) = binary_search((0, ()), (values.len(), ()), |i| {
            if values[i] < 6 {
                Direction::Low(())
            } else {
                Direction::High(())
            }
        });

        dbg!(largest_low);
        dbg!(smallest_high);
    }

    #[test]
    fn binary_search_witness_test() {
        let values = [Ok("foo"), Ok("bar"), Ok("baz"), Err(false), Err(true)];

        let (largest_low, smallest_high) =
            binary_search((0, "bar"), (values.len() - 1, true), |i| {
                match dbg!(values[dbg!(i)]) {
                    Ok(x) => Direction::Low(x),
                    Err(x) => Direction::High(x),
                }
            });

        dbg!(largest_low); // "baz"
        dbg!(smallest_high); // false
    }
}
