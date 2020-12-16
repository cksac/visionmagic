// Copyright 2020 Tsang Hao Fung. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use wasm_bindgen::prelude::*;

mod canvas;
mod common;
mod simplification;
mod segmentation;
mod utils;

#[wasm_bindgen(start)]
pub fn main() {
    utils::set_panic_hook();
    console_log::init().unwrap();
}