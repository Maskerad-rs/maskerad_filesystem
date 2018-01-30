// Copyright 2017-2018 Maskerad Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use std::fs;

// We need our own version of this structure because the one in
// std annoyingly doesn't let you get data out of it.
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct OpenOptions {
    read: bool,
    write: bool,
    create: bool,
    append: bool,
    truncate: bool,
}
impl OpenOptions {
    // Create a new instance
    pub fn new() -> OpenOptions {
        Default::default()
    }

    // Open for reading
    pub fn set_read(&mut self, read: bool) -> &mut OpenOptions {
        self.read = read;
        self
    }

    // Open for writing
    pub fn set_write(&mut self, write: bool) -> &mut OpenOptions {
        self.write = write;
        self
    }

    // Create the file if it does not exist yet
    pub fn set_create(&mut self, create: bool) -> &mut OpenOptions {
        self.create = create;
        self
    }

    // Append at the end of the file
    pub fn set_append(&mut self, append: bool) -> &mut OpenOptions {
        self.append = append;
        self
    }

    // Truncate the file to 0 bytes after opening
    pub fn set_truncate(&mut self, truncate: bool) -> &mut OpenOptions {
        self.truncate = truncate;
        self
    }

    pub fn to_fs_openoptions(&self) -> fs::OpenOptions {
        let mut opt = fs::OpenOptions::new();
        opt.read(self.read)
            .write(self.write)
            .create(self.create)
            .append(self.append)
            .truncate(self.truncate)
            .create(self.create);
        opt
    }
}
