// Copyright 2017-2018 Maskerad Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use std::io::{Read, Seek, Write};
use std::fmt;

/*

//We create a VFile trait to pave the way to different type of files.
pub trait VFile: Read + Seek + Write + fmt::Debug {}
//TODO: Think about the different types of files (StreamableTexture ? MusicFile ? ShaderFile ? LogFile ?)
impl<T: Read + Seek + Write + fmt::Debug> VFile for T {}

*/