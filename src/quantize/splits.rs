// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::Interval;
use ord_subset::OrdSubset;
use ord_subset::OrdSubsetIterExt;
use std::fmt::Debug;
use std::hash::Hash;
use std::collections::HashMap;

// `splits` is a list of indices where we want to break the values into intervals.
// The values are the (value, class) pairs in `data`, and the `splits` contents are indicies are into `data`.
// The first split is "anything below this value", and the last is "anything of this value and above".
// Anything else is a range interval.
// If there are no splits, then there's a single interval covering all values.
pub fn intervals_from_splits<A, C>(splits: Vec<usize>, data: &[(&A, &C)]) -> Vec<Interval<A, C>>
where
    A: OrdSubset + Copy + Debug,
    C: Eq + Hash + Copy + Debug,
{
    // What do do about ties for most frequent class? https://github.com/d6y/oner/issues/3#issuecomment-537864969
    let most_frequent_class = |start: usize, until: usize| {
        let classes: Vec<C> = data[start..until].iter().map(|pair| pair.1).cloned().collect();
        let largest: Option<C> = frequency_count(&classes)
            .into_iter()
            .ord_subset_max_by_key(|pair| pair.1)
            .map(|pair| *pair.0);

        largest.unwrap_or_else(|| panic!("Found no classes for a split during quantization. This is likely a bug in this quantize implementation. Range is {} until {} in splits {:?} for data {:?}", start, until, &splits, data))
    };

    let lower = |index: usize| Interval::Lower {
        below: data[index].0.to_owned(),
        class: most_frequent_class(0, index),
    };

    let upper = |index: usize| Interval::Upper {
        from: data[index].0.to_owned(),
        class: most_frequent_class(index, data.len()),
    };

    let range = |index_start: usize, index_end: usize| Interval::Range {
        from: data[index_start].0.to_owned(),
        below: data[index_end].0.to_owned(),
        class: most_frequent_class(index_start, index_end),
    };

    let infinite = || Interval::Infinite { class: most_frequent_class(0, data.len()) };

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

pub fn trim_splits<A, C: Eq + Hash>(splits: Vec<usize>, small: usize, data: &[(&A, &C)]) -> Vec<usize> {
    // Tail-recursive safe walk of the splits:
    trim_splits0(splits.as_slice(), small, data, Vec::new(), 0)
}

fn trim_splits0<A, C: Eq + Hash>(
    splits: &[usize],
    small: usize,
    data: &[(&A, &C)],
    mut keep: Vec<usize>,
    start_index: usize,
) -> Vec<usize> {
    if let Some(head) = splits.first() {
        let tail = &splits[1..];
        if no_dominant_class(start_index, *head, small, data) {
            // Drop this split:
            trim_splits0(tail, small, data, keep, start_index)
        } else {
            // Keep the split, and carry on from this point (`head`):
            keep.push(*head);
            trim_splits0(tail, small, data, keep, *head)
        }
    } else {
        // No more elements to process
        keep
    }
}

fn no_dominant_class<A, C: Eq + Hash>(
    start: usize,
    until: usize,
    small: usize,
    data: &[(&A, &C)],
) -> bool {
    let classes: Vec<&C> = data[start..until].iter().map(|pair| pair.1).collect();
    let counts = frequency_count(&classes);
    counts.values().all(|&count| count <= small)
}

fn frequency_count<T>(ts: &[T]) -> HashMap<&T, usize>
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