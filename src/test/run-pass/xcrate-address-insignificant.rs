// Copyright 2013 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// xfail-fast check-fast doesn't like aux-build
// aux-build:xcrate_address_insignificant.rs

extern mod foo = "xcrate_address_insignificant";

pub fn main() {
    assert_eq!(foo::foo::<f64>(), foo::bar());
}
