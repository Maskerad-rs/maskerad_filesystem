// Copyright 2017-2018 Maskerad Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

#[cfg(feature = "serde")]
#[macro_use]
extern crate serde;

extern crate remove_dir_all;

pub mod filesystem_error;
pub mod game_directories;
pub mod filesystem;
pub mod open_options;
