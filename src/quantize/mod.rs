// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::Interval;
use super::interval::merge_neighbours_with_same_class;
use ord_subset::OrdSubset;
use ord_subset::OrdSubsetSliceExt;
use std::fmt::Debug;
use std::hash::Hash;

mod splits;
use splits::{intervals_from_splits, trim_splits};

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

pub fn find_intervals<A, C>(column: &[A], classes: &[C], small: usize) -> Vec<Interval<A, C>>
where
    A: OrdSubset + Copy + Debug,
    C: Eq + Hash + Copy + Debug,
{
    // 1. Get the attribute values (plus associated class) in attribute sorted order:
    let mut sorted: Vec<(&A, &C)> = Vec::new();
    for (v, c) in column.iter().zip(classes.iter()) {
        sorted.push((v, c));
    }
    sorted.ord_subset_sort_by_key(|pair| pair.0);

    // 2. Create a split each time the classification changes
    let mut split_index = Vec::new(); // Index into `sorted` where the classification changes to a different value.
    for (prev_index, ((_cur_value, cur_class), (_prev_val, prev_class))) in
        sorted.iter().skip(1).zip(sorted.iter()).enumerate()
    {
        if cur_class != prev_class {
            split_index.push(prev_index + 1);
        }
    }

    // 3. Remove splits that are too small:
    let split_trimmed = trim_splits(split_index, small, &sorted);

    // 4. Generate distinct intervals from the splits:
    let intervals: Vec<Interval<A, C>> = intervals_from_splits(split_trimmed, &sorted);

    let merged_intervals = merge_neighbours_with_same_class(&intervals);
    merged_intervals
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
