// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! The 1R (Holt, 1993) quantization algorithm.
//!
//! Given an attribute contaning numeric data,
//! this code will compute a set of intervals to quantize the values.
//!
//! # Examples
//!
//! ```
//! use oner_quantize::*;
//!
//! // Fake data that has three clear splits:
//! let attribute = vec![  1, 10,   3,   1,  20,  30,  100];
//! let classes   = vec!["a", "b", "a", "a", "b", "b", "c"];
//!
//! // Discover the intervals:
//! let intervals =
//!    find_intervals(&attribute, &classes, 2);
//!
//! // We'll find three intervals:
//! // `< 10`, `>= 10 and < 100`, `>= 100`
//! assert_eq!(intervals.len(), 3);
//!
//! // Apply the intervals to an attribute value:
//! assert_eq!(
//!     quantize(&intervals, 47),
//!     Some( &Interval::Range { from: 10, below: 100, class: "b" } )
//! );
//!```
//!
//! # See also
//!
//! * [oner_induction crate](https://docs.rs/oner_induction) - a Rust implementation of the rule induction algorithm from the 1R paper.
//! * Holte, R.C. (1993) Very Simple Classification Rules Perform Well on Most Commonly Used Datasets. _Machine Learning_ 11: 63. <https://doi.org/10.1023/A:1022631118932>
//! * Nevill-Manning, C. G., Holmes, G. & Witten, I. H.(1995) The development of Holte's 1R Classifier. (Working paper 95/19). Hamilton, New Zealand: University of Waikato, Department of Computer Science. <https://hdl.handle.net/10289/1096>
//!
//! # Licence
//!
//! Copyright 2020 Richard Dallaway
//!
//! This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0.
//! If a copy of the MPL was not distributed with this file, You can obtain one at <https://mozilla.org/MPL/2.0/>.

mod interval;
pub use interval::Interval;

mod quantize;
pub use quantize::find_intervals;
pub use quantize::quantize;
