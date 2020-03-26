// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use super::Interval;
use std::fmt::Debug;

fn interval_merge<A, C>(previous: &Interval<A, C>, later: &Interval<A, C>) -> Interval<A, C>
where
    A: PartialEq + PartialOrd + Copy + Debug,
    C: PartialEq + Copy + Debug,
{
    match (previous, later) {
        (Interval::Lower { .. }, Interval::Range { below, class, .. }) => {
            Interval::lower(below.to_owned(), class.to_owned())
        }
        (Interval::Lower { .. }, Interval::Upper { class, .. }) => {
            Interval::Infinite { class: *class }
        }
        (Interval::Range { from, .. }, Interval::Range { below, class, .. }) => {
            Interval::Range { from: *from, below: *below, class: *class }
        }
        (Interval::Range { from, .. }, Interval::Upper { class, .. }) => {
            Interval::upper(*from, *class)
        }
        _ => panic!(
            "Merging {:?} with {:?} is not supported. This is likely a bug in oner_quantize",
            previous, later
        ),
    }
}

/// After constructing intervals, we merge together neighbouring intervals with the same predicted class.
pub fn merge_neighbours_with_same_class<A, C>(intervals: &[Interval<A, C>]) -> Vec<Interval<A, C>>
where
    A: PartialEq + PartialOrd + Copy + Debug,
    C: PartialEq + Copy + Debug,
{
    let mut merged: Vec<Interval<A, C>> = Vec::new();

    if let Some(head) = intervals.first() {
        let mut last_class = head.class();
        merged.push(*head);

        let tail = &intervals[1..];
        for interval in tail {
            let class = interval.class();
            if class == last_class {
                let updated = merged.pop().map(|last| interval_merge(&last, interval));
                updated.into_iter().for_each(|i| merged.push(i));
            } else {
                last_class = class;
                merged.push(*interval);
            }
        }
    }

    merged
}
