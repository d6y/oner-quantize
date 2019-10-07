use super::dataset::{Dataset, Example};
use super::interval::Interval;
use super::iter::{all_numeric_or_missing, count_distinct, frequency_count};
use ord_subset::OrdSubsetSliceExt;
use std::collections::HashMap;

fn is_numeric(values: &[&String]) -> bool {
    // From Holt p. 66:
    // "To be counted, in table 2, as continuous (column entitled "cont") an attribute must have more than six numerical values."
    all_numeric_or_missing(values) && count_distinct(values) > 6
}

pub fn quantize(dataset: Dataset<String, Example>) -> Dataset<String, Example> {
    let num_attrs = dataset.input_attribute_names.len();
    for index in 0..num_attrs {
        let mut column = Vec::new();
        for example in &dataset.examples {
            column.push(&example.attribute_values[index]);
        }
        println!(
            "{} {}",
            &dataset.input_attribute_names[index],
            is_numeric(&column[..])
        );
    }

    dataset
}

fn quantize_column<'v>(column: &'v [(&str, &str)], small: usize) -> HashMap<&'v str, String> {
    // 1. Get the attribute values in sorted order:
    let mut sorted: Vec<(f32, &str)> = Vec::new();
    for (v, c) in column {
        if let Ok(n) = v.parse::<f32>() {
            sorted.push((n, c));
        } else {
            unimplemented!(
                "Cannot yet quantize non-numeric values: https://github.com/d6y/oner/issues/1"
            );
        }
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
    let intervals: Vec<Interval<f32, &str>> = Interval::from_splits(split_trimmed, &sorted);
    let merged_intervals = Interval::merge_neighbours_with_same_class(&intervals);

    // 5. Generate a re-mapping table from each value we've seen to a new qualitized value:
    let interval = |value: f32| merged_intervals.iter().find(|i| i.matches(value));

    let mut remapping: HashMap<&str, String> = HashMap::new();

    let original_string_values = column.iter().map(|(k, _v)| k);
    let numeric_values = sorted.iter().map(|(k, _v)| k);
    for (numeric, value) in numeric_values.zip(original_string_values) {
        if let Some(ival) = interval(*numeric) {
            remapping.insert(value, ival.show());
        }
    }
    remapping
}

fn trim_splits(splits: Vec<usize>, small: usize, data: &[(f32, &str)]) -> Vec<usize> {
    // Tail-recursive safe walk of the splits:
    trim_splits0(splits.as_slice(), small, data, Vec::new(), 0)
}

fn trim_splits0(
    splits: &[usize],
    small: usize,
    data: &[(f32, &str)],
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

fn no_dominant_class(start: usize, until: usize, small: usize, data: &[(f32, &str)]) -> bool {
    let classes: Vec<&str> = data[start..until].iter().map(|pair| pair.1).collect();
    let counts = frequency_count(&classes);
    counts.values().all(|&count| count <= small)
}

#[cfg(test)]
mod test_quantize {
    use super::quantize_column;
    use std::collections::HashMap;
    #[test]
    fn test_golf_example() {
        // This example (inputs, and boundary points) comes from:
        // Nevill-Manning, Holmes & Witten (1995)  _The Development of Holte's 1R Classifier_, p. 2

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
    }
}
