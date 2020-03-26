// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::fmt::Debug;

mod merge;
pub use merge::merge_neighbours_with_same_class;

/// An interval represents a mapping from a range of values of type `A`, to a class, `C`.
///
/// # Examples
///
/// ```
/// use oner_quantize::Interval;
/// let interval = Interval::lower(100, "true");
/// assert_eq!(interval.matches(25), true);
/// assert_eq!(interval.matches(100), false);
/// assert_eq!(interval.matches(125), false);
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Interval<A, C> {
    /// A lower bound, such as `< 100`
    Lower { below: A, class: C },
    /// A half-open exclusive range, such as `>= 100 and < 200` aka `[100,200)`
    Range { from: A, below: A, class: C },
    /// An upper range, such as `>= 200`
    Upper { from: A, class: C },
    /// The interval that covers all values
    Infinite { class: C },
}

impl<A, C> Interval<A, C>
where
    A: Debug + PartialOrd + Copy,
    C: Debug + Copy,
{
    pub fn lower(below: A, class: C) -> Self {
        Interval::Lower { below, class }
    }

    pub fn upper(from: A, class: C) -> Self {
        Interval::Upper { from, class }
    }
    /*
    pub fn show(&self) -> String {
        match self {
            Interval::Lower { below, .. } => format!("< {}", below),
            Interval::Range { from, below, .. } => format!(">= {} and < {}", from, below),
            Interval::Upper { from, .. } => format!(">= {}", from),
            Interval::Infinite { .. } => String::from("any value"),
        }
    }*/

    /// Does the given `value` fall inside this interval?
    pub fn matches(&self, value: A) -> bool {
        match self {
            Interval::Lower { below, .. } => value < *below,
            Interval::Range { from, below, .. } => value >= *from && value < *below,
            Interval::Upper { from, .. } => value >= *from,
            Interval::Infinite { .. } => true,
        }
    }

    /// The class that this interval predicts.
    pub fn class(&self) -> &C {
        match self {
            Interval::Lower { class, .. } => class,
            Interval::Range { class, .. } => class,
            Interval::Upper { class, .. } => class,
            Interval::Infinite { class } => class,
        }
    }
}
