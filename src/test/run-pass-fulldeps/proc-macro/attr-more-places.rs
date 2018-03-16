// Copyright 2016 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// aux-build:nop-attr.rs
// ignore-stage1
#![feature(proc_macro)]

extern crate nop_attr;

use nop_attr::{nop_attr, no_output};

fn main() {
    #[nop_attr]
    let string = "Hello, world!";

    println!("{}", string);

    #[no_output]
    {
        unsafe { some_definitely_unknown_symbol_which_should_be_removed() };
    }
}

extern {
    #[no_output]
    fn some_definitely_unknown_symbol_which_should_be_removed();
}
