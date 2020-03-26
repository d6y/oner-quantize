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
//! TODO
//!
//! # See also
//!
//! * [oner_induction](https://docs.rs/oner_induction) - a Rust implementation of the rule induction algorithm from the 1R paper.
//! * Holte, R.C. (1993) Very Simple Classification Rules Perform Well on Most Commonly Used Datasets. _Machine Learning_ 11: 63. [https://doi.org/10.1023/A:1022631118932](https://doi.org/10.1023/A:1022631118932).
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
