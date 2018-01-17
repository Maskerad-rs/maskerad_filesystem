// Copyright 2017-2018 Maskerad Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

/*

use std::fs;

//Rust provides metadata about files. We provide a VMetadata trait to pave the way to different type of metadata about different files.
pub trait VMetadata {
    //Is it a directory ?
    fn is_dir(&self) -> bool;
    //Is it a file ?
    fn is_file(&self) -> bool;
    //The length of the thing.
    fn len(&self) -> u64;
    //Is the file read only ?
    fn is_read_only(&self) -> bool;
}

pub struct Metadata(pub fs::Metadata);
impl VMetadata for Metadata {
    fn is_dir(&self) -> bool {
        self.0.is_dir()
    }
    fn is_file(&self) -> bool {
        self.0.is_file()
    }
    fn len(&self) -> u64 {
        self.0.len()
    }
    fn is_read_only(&self) -> bool {
        self.0.permissions().readonly()
    }
}

*/