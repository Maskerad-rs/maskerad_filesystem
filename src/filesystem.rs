// Copyright 2017-2018 Maskerad Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use std::fs;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::io::{BufReader, BufWriter};

use filesystem_error::{FileSystemError, FileSystemResult};
use open_options::OpenOptions;
use remove_dir_all;

//Open to read file
//Open to write to file
//Create file if it doesn't exist
//Append to file
//Access to metadata

/*FILESYSTEM.

A filesystem must provide the following functionalities :
- Manipulating file names and paths.
- open close read write append create files and directory.
- scan content of directory.
_____________________________________________________________
- asynchronous I/O (streaming music or textures...). -> just use Rayon directly.

TODO: Take a look at how mio handle async io with TCP. Or future stuff.
_____________________________________________________________
*/

/*
pub fn read<T: Read>(from: &mut BufReader<T>, to: &mut [u8]) -> FileSystemResult<usize> {
    from.read(to)
        .map_err(|io_error| FileSystemError::from(io_error))
}

pub fn read_to_end<T: Read>(from: &mut BufReader<T>, to: &mut Vec<u8>) -> FileSystemResult<usize> {
    from.read_to_end(to)
        .map_err(|io_error| FileSystemError::from(io_error))
}

pub fn read_to_string<T: Read>(
    from: &mut BufReader<T>,
    to: &mut String,
) -> FileSystemResult<usize> {
    from.read_to_string(to)
        .map_err(|io_error| FileSystemError::from(io_error))
}

pub fn read_exact<T: Read>(from: &mut BufReader<T>, to: &mut [u8]) -> FileSystemResult<()> {
    from.read_exact(to)
        .map_err(|io_error| FileSystemError::from(io_error))
}

pub fn write<T: Write>(to: &mut BufWriter<T>, from: &[u8]) -> FileSystemResult<usize> {
    to.write(from)
        .map_err(|io_error| FileSystemError::from(io_error))
}

pub fn write_all<T: Write>(to: &mut BufWriter<T>, from: &[u8]) -> FileSystemResult<()> {
    to.write_all(from)
        .map_err(|io_error| FileSystemError::from(io_error))
}
*/

pub fn get_absolute_path<P: AsRef<Path>>(path: P) -> FileSystemResult<PathBuf> {
    fs::canonicalize(path.as_ref()).map_err(|io_error| FileSystemError::from(io_error))
}

//Open file at path with options
fn open_with_options<P: AsRef<Path>>(path: P, open_options: &OpenOptions) -> FileSystemResult<File> {
    open_options
        .to_fs_openoptions()
        .open(path.as_ref())
        .map_err(|io_error| FileSystemError::from(io_error))
}

//Open file at path to read
pub fn open<P: AsRef<Path>>(path: P) -> FileSystemResult<BufReader<File>> {
    let buf = open_with_options(path.as_ref(), OpenOptions::new().set_read(true))?;
    Ok(BufReader::new(buf))
}

//Open file at path for writing, truncates if file already exist
pub fn create<P: AsRef<Path>>(path: P) -> FileSystemResult<BufWriter<File>> {
    let buf = open_with_options(
        path.as_ref(),
        OpenOptions::new()
            .set_create(true)
            .set_write(true)
            .set_truncate(true),
    )?;
    Ok(BufWriter::new(buf))
}

//Open the file at path for appending, creating it if necessary
pub fn append<P: AsRef<Path>>(path: P) -> FileSystemResult<BufWriter<File>> {
    let buf = open_with_options(
        path.as_ref(),
        OpenOptions::new()
            .set_create(true)
            .set_append(true)
            .set_write(true),
    )?;
    Ok(BufWriter::new(buf))
}

//create directory at path
pub fn mkdir<P: AsRef<Path>>(path: P) -> FileSystemResult<()> {
    fs::DirBuilder::new()
        .recursive(true)
        .create(path.as_ref())
        .map_err(|io_error| FileSystemError::from(io_error))
}

//remove a file
pub fn rm<P: AsRef<Path>>(path: P) -> FileSystemResult<()> {
    if path.as_ref().is_dir() {
        fs::remove_dir(path.as_ref()).map_err(|io_error| FileSystemError::from(io_error))
    } else {
        fs::remove_file(path.as_ref()).map_err(|io_error| FileSystemError::from(io_error))
    }
}

//remove file or directory and all its contents
pub fn rmrf<P: AsRef<Path>>(path: P) -> FileSystemResult<()> {
    if path.as_ref().exists() {
        remove_dir_all::remove_dir_all(path.as_ref()).map_err(|io_error| FileSystemError::from(io_error))
    } else {
        Ok(())
    }
}

//Retrieve all file entries in the given directory (recursive).
pub fn read_dir<P: AsRef<Path>>(path: P) -> FileSystemResult<fs::ReadDir> {
    fs::read_dir(path.as_ref()).map_err(|io_error| FileSystemError::from(io_error))
}

/*
pub fn async_read<T: Read + Send>(
    from: &mut BufReader<T>,
    to: &mut [u8],
    thread_pool: &ThreadPool,
) -> FileSystemResult<usize> {
    thread_pool.install(|| read(from, to))
}

pub fn async_read_to_end<T: Read + Send>(
    from: &mut BufReader<T>,
    to: &mut Vec<u8>,
    thread_pool: &ThreadPool,
) -> FileSystemResult<usize> {
    thread_pool.install(|| read_to_end(from, to))
}

pub fn async_read_to_string<T: Read + Send>(
    from: &mut BufReader<T>,
    to: &mut String,
    thread_pool: &ThreadPool,
) -> FileSystemResult<usize> {
    thread_pool.install(|| read_to_string(from, to))
}

pub fn async_read_exact<T: Read + Send>(
    from: &mut BufReader<T>,
    to: &mut [u8],
    thread_pool: &ThreadPool,
) -> FileSystemResult<()> {
    thread_pool.install(|| read_exact(from, to))
}

pub fn async_write<T: Write + Send>(
    to: &mut BufWriter<T>,
    from: &[u8],
    thread_pool: &ThreadPool,
) -> FileSystemResult<usize> {
    thread_pool.install(|| write(to, from))
}

pub fn async_write_all<T: Write + Send>(
    to: &mut BufWriter<T>,
    from: &[u8],
    thread_pool: &ThreadPool,
) -> FileSystemResult<()> {
    thread_pool.install(|| write_all(to, from))
}
*/

#[cfg(test)]
mod filesystem_test {
    use super::*;
    use std::io::BufReader;
    use std::io::{Read, Write};
    use std::env;
    //use rayon::Configuration;
    //use rayon::ThreadPool;
    use game_directories::{GameDirectories, RootDir};

    #[test]
    fn filesystem_io_operations() {
        let game_dirs =
            GameDirectories::new("test_filesystem_maskerad", "Malkaviel")
                .expect("Couldn't create FS");

        let current_dir_dir_test = game_dirs
            .construct_path_from_root(RootDir::WorkingDirectory, "dir_test")
            .expect("Could not create current_dir_dir_test PathBuf");

        mkdir(current_dir_dir_test.as_path())
            .expect("Could not create dir with current_dir_dir_test as path");
        assert!(current_dir_dir_test.exists());

        //user logs
        let user_log_dir_test = game_dirs
            .construct_path_from_root(RootDir::UserLogRoot, "log_dir_test")
            .expect("Could not create user_log_dir_test");
        mkdir(user_log_dir_test.as_path())
            .expect("Could not create dir with user_log_dir_test as path");
        assert!(user_log_dir_test.exists());

        let file_test = game_dirs
            .construct_path_from_root(RootDir::UserLogRoot, "log_dir_test/file_test.txt")
            .expect("Could not create file_test.txt");
        let mut log_dir_bufwriter =
            create(file_test.as_path()).expect("Could not create log_dir_test/file_test.txt");

        log_dir_bufwriter.write_all(b"text_test\n").unwrap();

        /*
        let async_dir = game_dirs
            .construct_path_from_root(RootDir::UserLogRoot, "async_dir")
            .expect("Could not create async_dir");
        mkdir(async_dir.as_path()).expect("Could not create dir with async_dir as path");
        assert!(async_dir.exists());

        //test async functionalities.
        let thread_pool = Configuration::new()
            .build()
            .expect("Could not create the thread pool.");

        let async_log_dir_test = game_dirs
            .construct_path_from_root(RootDir::UserLogRoot, "async_dir/async_log_dir_test.txt")
            .expect("Could not create async_log_dir_test");
        {
            let mut log_bufwriter =
                create(async_log_dir_test.as_path()).expect("Could not create the bufwriter");


            thread_pool.install(|| {
                log_bufwriter.write(b"test_async_text_1\n").unwrap()
            });

            thread_pool.install(|| {
                log_bufwriter.write(b"test_async_text_2\n").unwrap()
            });

            thread_pool.install(|| {
                log_bufwriter.write(b"test_async_text_3\n").unwrap()
            });

        } //bufwriter dropped here, all the write calls will be executed.


        let mut bufreader_async =
            open(async_log_dir_test.as_path()).expect("Could not create bufreader");
        let mut content = String::new();


        thread_pool.install(|| {
            bufreader_async.read_to_string(&mut content).unwrap()
        });

        let mut lines = content.lines();
        assert_eq!(lines.next(), Some("test_async_text_1"));
        assert_eq!(lines.next(), Some("test_async_text_2"));
        assert_eq!(lines.next(), Some("test_async_text_3"));
        assert_eq!(lines.next(), None);

        //Metadata
        let file_metadata = async_log_dir_test
            .metadata()
            .expect("Couldn't get metadata");
        assert!(file_metadata.is_file());
        assert!(!file_metadata.is_dir());
        assert!(file_metadata.len() > 0);

        //remove
        rm(async_log_dir_test.as_path())
            .expect("Couldn't delete the file : async_dir/async_log_dir_test.txt");
        assert!(!async_log_dir_test.exists());
        rmrf(current_dir_dir_test.as_path()).expect("Couldn't delete dir");
        assert!(!current_dir_dir_test.exists());
        */
    }

    #[test]
    fn filesystem_read_dir() {
        let game_dirs =
            GameDirectories::new("test_filesystem_blacksmith", "Malkaviel")
                .expect("Couldn't create GameDirs");
        let src_dir = game_dirs
            .construct_path_from_root(RootDir::WorkingDirectory, "src")
            .unwrap();
        let mut entries = read_dir(src_dir).unwrap();
        assert!(entries.next().is_some());
    }
}
