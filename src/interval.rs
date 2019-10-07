use super::iter::frequency_count;
use ord_subset::OrdSubsetIterExt;
use std::fmt::Debug;
use std::fmt::Display;
use std::hash::Hash;

#[derive(Debug, Clone, Copy)]
pub enum Interval<T, C> {
    Lower { below: T, class: C },          // e.g., < 100
    Range { from: T, below: T, class: C }, // e.g., >= 100 and < 200
    Upper { from: T, class: C },           // e.g., >= 200
    Infinite { class: C },
}

impl<T, C> Interval<T, C>
where
    T: Copy + Debug + Display + PartialOrd,
    C: Copy + Debug,
{
    pub fn show(&self) -> String {
        match self {
            Interval::Lower { below, .. } => format!("< {}", below),
            Interval::Range { from, below, .. } => format!(">= {} and < {}", from, below),
            Interval::Upper { from, .. } => format!(">= {}", from),
            Interval::Infinite { .. } => String::from("any value"),
        }
    }

    pub fn matches(&self, value: T) -> bool {
        match self {
            Interval::Lower { below, .. } => value < *below,
            Interval::Range { from, below, .. } => value >= *from && value < *below,
            Interval::Upper { from, .. } => value >= *from,
            Interval::Infinite { .. } => true,
        }
    }

    fn class(&self) -> &C {
        match self {
            Interval::Lower { class, .. } => class,
            Interval::Range { class, .. } => class,
            Interval::Upper { class, .. } => class,
            Interval::Infinite { class } => class,
        }
    }

    fn merge(&self, later: &Self) -> Self {
        match (self, later) {
            (Interval::Lower { .. }, Interval::Range { below, class, .. }) => Interval::Lower {
                below: *below,
                class: *class,
            },
            (Interval::Lower { .. }, Interval::Upper { class, .. }) => {
                Interval::Infinite { class: *class }
            }
            (Interval::Range { from, .. }, Interval::Range { below, class, .. }) => {
                Interval::Range {
                    from: *from,
                    below: *below,
                    class: *class,
                }
            }
            (Interval::Range { from, .. }, Interval::Upper { class, .. }) => Interval::Upper {
                from: *from,
                class: *class,
            },
            _ => panic!("Merging {:?} with {:?} is not supported", self, later),
        }
    }
}

impl<T, C> Interval<T, C>
where
    T: Copy + Debug + Display + PartialOrd,
    C: Copy + Debug + Eq + Hash,
{
    // `splits` is a list of indices where we want to break the values into intervals.
    // The values are the (value, class) pairs in `data`, and the `splits` contents are indicies are into `data`.
    // The first split is "anything below this value", and the last is "anything of this value and above".
    // Anything else is a range interval.
    // If there are no splits, then there's a single interval covering all values.
    pub fn from_splits(splits: Vec<usize>, data: &[(T, C)]) -> Vec<Interval<T, C>> {
        // What do do about ties for most frequent class? https://github.com/d6y/oner/issues/3#issuecomment-537864969
        let most_frequent_class = |start: usize, until: usize| {
            let classes: Vec<C> = data[start..until].iter().map(|pair| pair.1).collect();
            let largest: Option<&C> = frequency_count(&classes)
                .into_iter()
                .ord_subset_max_by_key(|pair| pair.1)
                .map(|pair| pair.0);

            *largest.unwrap_or_else(|| panic!("Found no classes for a split during quantization. Range is {} until {} in splits {:?} for data {:?}", start, until, &splits, data))
        };

        let lower = |index: usize| Interval::Lower {
            below: data[index].0,
            class: most_frequent_class(0, index),
        };

        let upper = |index: usize| Interval::Upper {
            from: data[index].0,
            class: most_frequent_class(index, data.len()),
        };

        let range = |index_start: usize, index_end: usize| Interval::Range {
            from: data[index_start].0,
            below: data[index_end].0,
            class: most_frequent_class(index_start, index_end),
        };

        let infinite = || Interval::Infinite {
            class: most_frequent_class(0, data.len()),
        };

        match splits.len() {
            0 => vec![infinite()],
            1 => vec![lower(splits[0]), upper(splits[0])],
            n => {
                let mut intervals = Vec::with_capacity(n + 1);
                intervals.push(lower(splits[0]));
                for (&curr_i, &prev_i) in splits.iter().skip(1).take(n - 1).zip(splits.iter()) {
                    intervals.push(range(prev_i, curr_i));
                }
                intervals.push(upper(splits[n - 1]));
                intervals
            }
        }
    }

    pub fn merge_neighbours_with_same_class(intervals: &[Interval<T, C>]) -> Vec<Interval<T, C>> {
        let mut merged: Vec<Interval<T, C>> = Vec::new();

        if let Some(head) = intervals.first() {
            let mut last_class = head.class();
            merged.push(*head);

            let tail = &intervals[1..];
            for interval in tail {
                let class = interval.class();
                if class == last_class {
                    let updated = merged.pop().map(|last| last.merge(interval));
                    updated.into_iter().for_each(|i| merged.push(i));
                } else {
                    last_class = class;
                    merged.push(*interval);
                }
            }
        }

        merged
    }
}
