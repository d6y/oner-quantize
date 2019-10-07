use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::Hash;
use std::str::FromStr;

pub fn frequency_count<T>(ts: &[T]) -> HashMap<&T, usize>
where
    T: Eq + Hash,
{
    let mut counts = HashMap::new();
    for t in ts {
        let count = counts.entry(t).or_insert(0);
        *count += 1;
    }
    counts
}

pub fn count_distinct<T>(xs: &[T]) -> usize
where
    T: Eq + Hash,
{
    let set: HashSet<&T> = xs.iter().collect();
    set.len()
}

pub fn all_numeric_or_missing(xs: &[&String]) -> bool {
    xs.iter().all(|x| x == &"" || x == &"?" || f32::from_str(x).is_ok())
}

#[cfg(test)]
mod test_iters {
    use super::count_distinct;
    #[test]
    fn test_count_distinct() {
        assert_eq!(0, count_distinct::<u8>(&[]));
        assert_eq!(1, count_distinct(&[0]));
        assert_eq!(1, count_distinct(&[0, 0]));
        assert_eq!(2, count_distinct(&[0, 1, 0]));
    }
}
