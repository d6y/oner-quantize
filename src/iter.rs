// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::collections::HashMap;
use std::hash::Hash;

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
