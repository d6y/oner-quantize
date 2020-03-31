// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use super::interval::merge_neighbours_with_same_class;
use crate::Interval;
use ord_subset::OrdSubset;
use ord_subset::OrdSubsetSliceExt;
use std::fmt::Debug;
use std::hash::Hash;

mod splits;
use splits::{intervals_from_splits, trim_splits};

/// Quantize the given `attribute` (aka feature, column) into an ordered list of `Intervals`.
///
/// # Arguments
///
/// * `attribute` - a single attribute, typically numeric, to be quantized.
/// * `classes` - the corresponsing class for each attribute value.
/// * `small` -  the small disjunct threshold, such as 3. There has to be at least one class in an interval with more than `small` values in the interval.
///
/// # Examples
/// ```
/// use oner_quantize::find_intervals;
/// use oner_quantize::Interval;
/// use oner_quantize::Interval::{Lower, Range, Upper};
///
/// // Fake data that has three clear splits:
/// let attribute = vec![  1, 10,   3,   1,  20,  30,  100];
/// let classes   = vec!["a", "b", "a", "a", "b", "b", "c"];
///
/// let intervals =
///    find_intervals(&attribute, &classes, 2);
///
/// assert_eq!(intervals, vec![
///   Lower { below: 10, class: "a" },
///   Range { from: 10, below: 100, class: "b" },
///   Upper { from: 100, class: "c" }
/// ]);
/// ```
pub fn find_intervals<A, C>(attribute: &[A], classes: &[C], small: usize) -> Vec<Interval<A, C>>
where
    A: OrdSubset + Copy + Debug,
    C: Eq + Hash + Copy + Debug,
{
    // 1. Get the attribute values (plus associated class) in attribute sorted order:
    let mut sorted: Vec<(&A, &C)> = Vec::new();
    for (v, c) in attribute.iter().zip(classes.iter()) {
        sorted.push((v, c));
    }
    sorted.ord_subset_sort_by_key(|pair| pair.0);

    // 2. Create a (tentative) split each time the attribute value changes.

    // Index into `sorted` where the classification changes to a different value.
    // That is, a value of 1 in `split_index` means the attribute value at sorted[0] differs from sorted[1].
    // The split happens between index 0 and 1 in that example.
    let mut split_index = Vec::new();
    for (prev_index, ((cur_value, _cur_class), (prev_value, _prev_class))) in
        sorted.iter().skip(1).zip(sorted.iter()).enumerate()
    {
        if cur_value != prev_value {
            split_index.push(prev_index + 1);
        }
    }

    // 3. Remove splits that are too small:
    let split_index_trimmed = trim_splits(split_index, small, &sorted);

    // 4. Generate distinct intervals from the splits:
    let intervals: Vec<Interval<A, C>> = intervals_from_splits(split_index_trimmed, &sorted);

    merge_neighbours_with_same_class(&intervals)
}

#[cfg(test)]
mod tests {
    use super::find_intervals;
    use super::Interval;
    #[test]
    fn test_golf_example() {
        // This example (inputs, and boundary points) comes from:
        // Nevill-Manning, Holmes & Witten (1995)  _The Development of Holte's 1R Classifier_, p. 2

        let attrbibute = vec![64, 65, 68, 69, 70, 71, 72, 72, 75, 75, 80, 81, 83, 85];

        let classes = vec!["p", "d", "p", "p", "p", "d", "p", "d", "p", "p", "d", "p", "p", "d"];

        let actual = find_intervals(&attrbibute, &classes, 3);

        let expected = vec![Interval::lower(85, "p"), Interval::upper(85, "d")];

        assert_eq!(expected, actual);

        /*
        let inputs = [
            ("64", "P"),
            ("65", "D"),
            ("68", "P"),
            ("69", "P"),
            ("70", "P"),
            ("71", "D"),
            ("72", "P"),
            ("72", "D"),
            ("75", "P"),
            ("75", "P"),
            ("80", "D"),
            ("81", "P"),
            ("83", "P"),
            ("85", "D"),
        ];

        let i1 = "< 85";
        let i2 = ">= 85";

        let expected: HashMap<&str, String> = [
            ("64", i1),
            ("65", i1),
            ("68", i1),
            ("69", i1),
            ("70", i1),
            ("71", i1),
            ("72", i1),
            ("75", i1),
            ("75", i1),
            ("80", i1),
            ("81", i1),
            ("83", i1),
            ("85", i2),
        ]
        .iter()
        .map(|(v, s)| (*v, s.to_string()))
        .collect();
        assert_eq!(expected, quantize_column(&inputs, 3));
        */
    }
}

/*
    // 5. Generate a re-mapping table from each value we've seen to a new qualitized value:
    let interval = |value: &A| merged_intervals.iter().find(|i| i.matches(value));

    let mut remapping: HashMap<&str, String> = HashMap::new();

    let original_string_values = column.iter().map(|(k, _v)| k);
    let numeric_values = sorted.iter().map(|(k, _v)| k);
    for (numeric, value) in numeric_values.zip(original_string_values) {
        if let Some(ival) = interval(*numeric) {
            remapping.insert(value, ival.show());
        }
    }
*/
