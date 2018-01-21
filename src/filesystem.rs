// Copyright 2017-2018 Maskerad Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use std::fs;
use std::fs::File;
use std::fmt;
use std::path::{Path, PathBuf};
use std::io::{BufReader, BufWriter, Read, Write};

use game_infos::GameInfos;
use game_directories::{GameDirectories, RootDir};
use filesystem_error::{FileSystemResult, FileSystemError};
use rayon::{ThreadPool, Configuration};
use open_options::OpenOptions;
use file_extension::FileExtension;
//use files::VFile;
//use metadata::{VMetadata, Metadata};
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
- does a file or directory exists ?.
- get metadata about files.
- scan content of directory.
- asynchronous I/O (streaming music or textures...).
*/

//TODO: Still not sure about the way to create the bufreader and writers, and about the async stuff.

fn get_absolute_path(root_dir: &PathBuf, path: &str) -> PathBuf {
    let mut root = root_dir.clone();
    //An empty &str can be used to delete a root directory (for tests). A bit hacky but....
    if !path.is_empty() {
        root.push(Path::new(path));
    }
    root
}

fn get_extension(path: &str) -> FileSystemResult<FileExtension> {
    match Path::new(path).extension() {
        Some(extension) => {
            match extension.to_str().expect("Not valid unicode") {
                "gltf" => {
                    Ok(FileExtension::GLTF)
                },
                "flac" => {
                    Ok(FileExtension::FLAC)
                },
                "ogg" => {
                    Ok(FileExtension::OGG)
                },
                "tga" => {
                    Ok(FileExtension::TGA)
                },
                "toml" => {
                    Ok(FileExtension::TOML)
                }
                _ => {
                    Err(FileSystemError::ExtensionError(format!("The file extension {:?} at path {} isn't a supported file extension (tga, flac, ogg, gltf, toml).", extension, path)))
                }
            }
        },
        None => {
            Err(FileSystemError::ExtensionError(format!("The path {} doesn't have a valid extension ! No file name ? No embedded '.' ? Begins with a '.' but doesn't have other '.' within ?", path)))
        }
    }
}

//Open file at path with options
fn open_with_options(root_dir: &PathBuf, path: &str, open_options: &OpenOptions) -> FileSystemResult<File> {
    let absolute_path = get_absolute_path(root_dir, path);

    open_options
        .to_fs_openoptions()
        .open(absolute_path.as_path())
        .map_err(|io_error| {
            FileSystemError::from(io_error)
        })
}

fn open_as_bufreader(root_dir: &PathBuf, path: &str) -> FileSystemResult<BufReader<File>> {
    let buf = open_with_options(root_dir, path, OpenOptions::new().set_read(true))?;
    Ok(BufReader::new(buf))
}

//Open file at path for writing, truncates if file already exist
fn create_as_bufwriter(root_dir: &PathBuf, path: &str) -> FileSystemResult<BufWriter<File>> {
    let buf = open_with_options( root_dir, path, OpenOptions::new().set_create(true).set_write(true).set_truncate(true))?;
    Ok(BufWriter::new(buf))
}

//Open the file at path for appending, creating it if necessary
fn append_as_bufwriter(root_dir: &PathBuf, path: &str) -> FileSystemResult<BufWriter<File>> {
    let buf = open_with_options(root_dir, path, OpenOptions::new().set_create(true).set_append(true).set_write(true))?;
    Ok(BufWriter::new(buf))
}

//create directory at path
fn mkdir(root_dir: &PathBuf, path: &str) -> FileSystemResult<()> {
    let absolute_path = get_absolute_path(root_dir, path);
    fs::DirBuilder::new().recursive(true).create(absolute_path.as_path()).map_err(|io_error| {
        FileSystemError::from(io_error)
    })
}
//remove a file
fn rm(root_dir: &PathBuf, path: &str) -> FileSystemResult<()> {
    let absolute_path = get_absolute_path(root_dir, path);

    if absolute_path.is_dir() {
        fs::remove_dir(absolute_path).map_err(|io_error| {
            FileSystemError::from(io_error)
        })
    } else {
        fs::remove_file(absolute_path).map_err(|io_error| {
            FileSystemError::from(io_error)
        })
    }
}
//remove file or directory and all its contents
fn rmrf(root_dir: &PathBuf, path: &str) -> FileSystemResult<()> {
    if exists(root_dir, path) {
        let absolute_path = get_absolute_path(root_dir, path);
        remove_dir_all::remove_dir_all(absolute_path.as_path()).map_err(|io_error| {
            FileSystemError::from(io_error)
        })
    } else {
        Ok(())
    }
}
//Check if file exists
fn exists(root_dir: &PathBuf, path: &str) -> bool {
    get_absolute_path(root_dir, path).exists()
}

//Get file's metadata
fn metadata(root_dir: &PathBuf, path: &str) -> FileSystemResult<fs::Metadata> {
    let absolute_path = get_absolute_path(root_dir, path);
    absolute_path.metadata().map_err(|error| {
        FileSystemError::from(error)
    })
}

//Retrieve all file entries in the given directory (recursive).
fn read_dir(root_dir: &PathBuf, path: &str) -> FileSystemResult<fs::ReadDir> {
    let absolute_path = get_absolute_path(root_dir, path);

    fs::read_dir(absolute_path.as_path()).map_err(|io_error| {
        FileSystemError::from(io_error)
    })
}

fn read<T: Read>(bufreader: &mut BufReader<T>, buf: &mut [u8]) -> FileSystemResult<usize> {
    bufreader.read(buf).map_err(|io_error| {
        FileSystemError::from(io_error)
    })
}

fn read_to_end<T: Read>(bufreader: &mut BufReader<T>, buf: &mut Vec<u8>) -> FileSystemResult<usize> {
    bufreader.read_to_end(buf).map_err(|io_error| {
        FileSystemError::from(io_error)
    })
}

fn read_to_string<T: Read>(bufreader: &mut BufReader<T>, buf: &mut String) -> FileSystemResult<usize> {
    bufreader.read_to_string(buf).map_err(|io_error| {
        FileSystemError::from(io_error)
    })
}

fn read_exact<T: Read>(bufreader: &mut BufReader<T>, buf: &mut [u8]) -> FileSystemResult<()> {
    bufreader.read_exact(buf).map_err(|io_error| {
        FileSystemError::from(io_error)
    })
}

fn write<T: Write>(bufwriter: &mut BufWriter<T>, buf: &[u8]) -> FileSystemResult<usize> {
    bufwriter.write(buf).map_err(|io_error| {
        FileSystemError::from(io_error)
    })
}

fn write_all<T: Write>(bufwriter: &mut BufWriter<T>, buf: &[u8]) -> FileSystemResult<()> {
    bufwriter.write_all(buf).map_err(|io_error| {
        FileSystemError::from(io_error)
    })
}





fn async_read<T: Read + Send>(bufreader: &mut BufReader<T>, buf: &mut [u8], thread_pool: &ThreadPool) -> FileSystemResult<usize> {
    thread_pool.install(|| {
        read(bufreader, buf)
    })
}

fn async_read_to_end<T: Read + Send>(bufreader: &mut BufReader<T>, buf: &mut Vec<u8>, thread_pool: &ThreadPool) -> FileSystemResult<usize> {
    thread_pool.install(|| {
        read_to_end(bufreader, buf)
    })
}

fn async_read_to_string<T: Read + Send>(bufreader: &mut BufReader<T>, buf: &mut String, thread_pool: &ThreadPool) -> FileSystemResult<usize> {
    thread_pool.install(|| {
        read_to_string(bufreader, buf)
    })
}

fn async_read_exact<T: Read + Send>(bufreader: &mut BufReader<T>, buf: &mut [u8], thread_pool: &ThreadPool) -> FileSystemResult<()> {
    thread_pool.install(|| {
        read_exact(bufreader, buf)
    })
}

fn async_write<T: Write + Send>(bufwriter: &mut BufWriter<T>, buf: &[u8], thread_pool: &ThreadPool) -> FileSystemResult<usize> {
    thread_pool.install(|| {
        write(bufwriter, buf)
    })
}

fn async_write_all<T: Write + Send>(bufwriter: &mut BufWriter<T>, buf: &[u8], thread_pool: &ThreadPool) -> FileSystemResult<()> {
    thread_pool.install(|| {
        write_all(bufwriter, buf)
    })
}




pub struct FileSystem {
    game_directories: GameDirectories,
}

impl FileSystem {
    pub fn new(game_infos: GameInfos) -> FileSystemResult<Self> {
        let game_dirs = GameDirectories::new(game_infos)?;

        Ok(FileSystem {
            game_directories: game_dirs,
        })
    }

    pub fn get_absolute_path(&self, root_dir: &RootDir, path: &str) -> FileSystemResult<PathBuf> {
        let root_dir = self.game_directories.path(root_dir)?;
        Ok(get_absolute_path(root_dir, path))
    }

    pub fn get_file_extension(&self, path: &str) -> FileSystemResult<FileExtension> {
        get_extension(path)
    }

    //Open file at path to read
    pub fn open(&self, root_dir: &RootDir, path: &str) -> FileSystemResult<BufReader<File>> {
        open_as_bufreader(self.game_directories.path(root_dir)?, path)
    }

    //Open file at path for writing, truncates if file already exist
    pub fn create(&self, root_dir: &RootDir, path: &str) -> FileSystemResult<BufWriter<File>> {
        create_as_bufwriter(self.game_directories.path(root_dir)?, path)
    }

    //Open the file at path for appending, creating it if necessary
    pub fn append(&self, root_dir: &RootDir, path: &str) -> FileSystemResult<BufWriter<File>> {
        append_as_bufwriter(self.game_directories.path(root_dir)?, path)
    }

    //create directory at path
    pub fn mkdir(&self, root_dir: &RootDir, path: &str) -> FileSystemResult<()> {
        mkdir(self.game_directories.path(root_dir)?, path)
    }

    //remove a file
    pub fn rm(&self, root_dir: &RootDir, path: &str) -> FileSystemResult<()> {
        rm(self.game_directories.path(root_dir)?, path)
    }

    //remove file or directory and all its contents
    pub fn rmrf(&self, root_dir: &RootDir, path: &str) -> FileSystemResult<()> {
        rmrf(self.game_directories.path(root_dir)?, path)
    }

    //Check if file exists
    pub fn exists(&self, root_dir: &RootDir, path: &str) -> bool {
        exists(self.game_directories.path(root_dir).unwrap(), path)
    }

    //Get file's metadata
    pub fn metadata(&self, root_dir: &RootDir, path: &str) -> FileSystemResult<fs::Metadata> {
        metadata(self.game_directories.path(root_dir)?, path)
    }

    //Retrieve all file entries in the given directory (recursive).
    pub fn read_dir(&self, root_dir: &RootDir, path: &str) -> FileSystemResult<fs::ReadDir> {
        read_dir(self.game_directories.path(root_dir)?, path)
    }

    pub fn read<T: Read>(&self, bufreader: &mut BufReader<T>, buf: &mut [u8]) -> FileSystemResult<usize> {
        read(bufreader, buf)
    }

    pub fn read_to_end<T: Read>(&self, bufreader: &mut BufReader<T>, buf: &mut Vec<u8>) -> FileSystemResult<usize> {
        read_to_end(bufreader, buf)
    }

    pub fn read_to_string<T: Read>(&self, bufreader: &mut BufReader<T>, buf: &mut String) -> FileSystemResult<usize> {
        read_to_string(bufreader, buf)
    }

    pub fn read_file_exact<T: Read>(&self, bufreader: &mut BufReader<T>, buf: &mut [u8]) -> FileSystemResult<()> {
        read_exact(bufreader, buf)
    }

    pub fn write<T: Write>(&self, bufwriter: &mut BufWriter<T>, buf: &[u8]) -> FileSystemResult<usize> {
        write(bufwriter, buf)
    }

    pub fn write_all<T: Write>(&self, bufwriter: &mut BufWriter<T>, buf: &[u8]) -> FileSystemResult<()> {
        write_all(bufwriter, buf)
    }

    fn async_read<T: Read + Send>(&self, bufreader: &mut BufReader<T>, buf: &mut [u8], thread_pool: &ThreadPool) -> FileSystemResult<usize> {
        async_read(bufreader, buf, thread_pool)
    }

    fn async_read_to_end<T: Read + Send>(&self, bufreader: &mut BufReader<T>, buf: &mut Vec<u8>, thread_pool: &ThreadPool) -> FileSystemResult<usize> {
        async_read_to_end(bufreader, buf, thread_pool)
    }

    fn async_read_to_string<T: Read + Send>(&self, bufreader: &mut BufReader<T>, buf: &mut String, thread_pool: &ThreadPool) -> FileSystemResult<usize> {
        async_read_to_string(bufreader, buf, thread_pool)
    }

    fn async_read_exact<T: Read + Send>(&self, bufreader: &mut BufReader<T>, buf: &mut [u8], thread_pool: &ThreadPool) -> FileSystemResult<()> {
        async_read_exact(bufreader, buf, thread_pool)
    }

    fn async_write<T: Write + Send>(&self, bufwriter: &mut BufWriter<T>, buf: &[u8], thread_pool: &ThreadPool) -> FileSystemResult<usize> {
        async_write(bufwriter, buf, thread_pool)
    }

    fn async_write_all<T: Write + Send>(&self, bufwriter: &mut BufWriter<T>, buf: &[u8], thread_pool: &ThreadPool) -> FileSystemResult<()> {
        async_write_all(bufwriter, buf, thread_pool)
    }
}


#[cfg(test)]
mod filesystem_test {
    use super::*;
    use std::io::BufReader;
    use std::io::Read;
    use std::env;

    #[test]
    fn filesystem_io_operations() {
        let filesystem = FileSystem::new(GameInfos::new("test_filesystem_maskerad", "Malkaviel")).expect("Couldn't create FS");

        filesystem.mkdir(&RootDir::WorkingDirectory, "dir_test").unwrap();
        assert!(filesystem.exists(&RootDir::WorkingDirectory, "dir_test"));

        //user logs
        filesystem.mkdir(&RootDir::UserLogRoot, "log_dir_test").unwrap();
        assert!(filesystem.exists(&RootDir::UserLogRoot, "log_dir_test"));
        let mut log_dir_bufwriter = filesystem.create(&RootDir::UserLogRoot, "log_dir_test/file_test.txt").expect("Could not create log_dir_test/file_test.txt");
        filesystem.write_all(&mut log_dir_bufwriter, b"text_test\n").expect("Couldn't add 'text test'");



        filesystem.mkdir(&RootDir::UserLogRoot, "async_dir").unwrap();
        assert!(filesystem.exists(&RootDir::UserLogRoot, "async_dir"));
        //test async functionalities.
        let thread_pool = Configuration::new().build().expect("Could not create the thread pool.");

        {
            let mut log_bufwriter = filesystem.create(&RootDir::UserLogRoot, "async_dir/async_log_dir_test.txt").expect("Could not create the bufwriter");
            filesystem.async_write_all(&mut log_bufwriter, b"test_async_text_1\n", &thread_pool).expect("Could not write to file async_log_dir_test and write test_async_test_1 asynchronously");
            filesystem.async_write_all(&mut log_bufwriter, b"test_async_text_2\n", &thread_pool).expect("Could not write to file async_log_dir_test and write test_async_test_2 asynchronously");
            filesystem.async_write_all(&mut log_bufwriter, b"test_async_text_3\n", &thread_pool).expect("Could not write to file async_log_dir_test and write test_async_test_3 asynchronously");
        } //bufwriter dropped here, all the write calls will be executed.

        let mut bufreader_async = filesystem.open(&RootDir::UserLogRoot, "async_dir/async_log_dir_test.txt").expect("Could not create bufreader");
        let mut content = String::new();
        filesystem.async_read_to_string(&mut bufreader_async, &mut content, &thread_pool).unwrap();
        let mut lines = content.lines();
        assert_eq!(lines.next(), Some("test_async_text_1"));
        assert_eq!(lines.next(), Some("test_async_text_2"));
        assert_eq!(lines.next(), Some("test_async_text_3"));
        assert_eq!(lines.next(), None);

        //Metadata
        let file_metadata = filesystem.metadata(&RootDir::UserLogRoot, "async_dir/async_log_dir_test.txt").expect("Couldn't get metadata");
        assert!(file_metadata.is_file());
        assert!(!file_metadata.is_dir());
        assert!(file_metadata.len() > 0);

        //remove
        filesystem.rm(&RootDir::UserLogRoot, "async_dir/async_log_dir_test.txt").expect("Couldn't delete the file : async_dir/async_log_dir_test.txt");
        assert!(!filesystem.exists(&RootDir::UserLogRoot, "async_dir/async_log_dir_test.txt"));
        filesystem.rmrf(&RootDir::WorkingDirectory, "dir_test").expect("Couldn't delete dir");
        assert!(!filesystem.exists(&RootDir::WorkingDirectory, "dir_test"));
    }


    #[test]
    fn filesystem_read_dir() {
        let filesystem = FileSystem::new(GameInfos::new("test_filesystem_blacksmith", "Malkaviel")).expect("Couldn't create FS");
        let mut entries = filesystem.read_dir(&RootDir::WorkingDirectory, "src").unwrap();
        assert!(entries.next().is_some());
    }
}