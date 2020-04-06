![Rust](https://github.com/d6y/oner_quantize/workflows/Rust/badge.svg)

# 1R quantization implementation in Rust

Quantization takes numeric data and turns it into a discrete set of intervals.

For example, given labelled data such:

| Value | Label   |
|-------| ------- |
| 1     | true |
| 40    | true |
| 100   | false |
| 101   | false |

We might discover the intervals:

- less than 100 (true)
- 100 or more (false)

This is a reimplementation of the 1R quantization algorithm described in [Holte (1993)](https://link.springer.com/article/10.1023%2FA%3A1022631118932). It is a complement to <https://crates.io/crates/oner_induction>.

# Documentation and examples

- [API reference and usage](https://docs.rs/oner_quantize/)
- An example application: <https://github.com/d6y/oner>

# License

Copyright 2020 Richard Dallaway

This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at <https://mozilla.org/MPL/2.0/>.


