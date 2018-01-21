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
//TODO: We assume that the &Path given to the functions is an absolute Path.

fn get_absolute_path(path: &Path) -> FileSystemResult<PathBuf> {
    fs::canonicalize(path).map_err(|io_error| {
        FileSystemError::from(io_error)
    })
}

fn get_extension(path: &Path) -> FileSystemResult<FileExtension> {
    match path.extension() {
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
                    Err(FileSystemError::ExtensionError(format!("The file extension {:?} at path {:?} isn't a supported file extension (tga, flac, ogg, gltf, toml).", extension, path)))
                }
            }
        },
        None => {
            Err(FileSystemError::ExtensionError(format!("The path {:?} doesn't have a valid extension ! No file name ? No embedded '.' ? Begins with a '.' but doesn't have other '.' within ?", path)))
        }
    }
}

//Open file at path with options
fn open_with_options(path: &Path, open_options: &OpenOptions) -> FileSystemResult<File> {

    open_options
        .to_fs_openoptions()
        .open(path)
        .map_err(|io_error| {
            FileSystemError::from(io_error)
        })
}

fn open_as_bufreader(path: &Path) -> FileSystemResult<BufReader<File>> {
    let buf = open_with_options(path, OpenOptions::new().set_read(true))?;
    Ok(BufReader::new(buf))
}

//Open file at path for writing, truncates if file already exist
fn create_as_bufwriter(path: &Path) -> FileSystemResult<BufWriter<File>> {
    let buf = open_with_options(path, OpenOptions::new().set_create(true).set_write(true).set_truncate(true))?;
    Ok(BufWriter::new(buf))
}

//Open the file at path for appending, creating it if necessary
fn append_as_bufwriter(path: &Path) -> FileSystemResult<BufWriter<File>> {
    let buf = open_with_options(path, OpenOptions::new().set_create(true).set_append(true).set_write(true))?;
    Ok(BufWriter::new(buf))
}





//create directory at path
fn mkdir(path: &Path) -> FileSystemResult<()> {
    fs::DirBuilder::new().recursive(true).create(path).map_err(|io_error| {
        FileSystemError::from(io_error)
    })
}
//remove a file
fn rm(path: &Path) -> FileSystemResult<()> {

    if path.is_dir() {
        fs::remove_dir(path).map_err(|io_error| {
            FileSystemError::from(io_error)
        })
    } else {
        fs::remove_file(path).map_err(|io_error| {
            FileSystemError::from(io_error)
        })
    }
}
//remove file or directory and all its contents
fn rmrf(path: &Path) -> FileSystemResult<()> {
    if exists(path) {
        remove_dir_all::remove_dir_all(path).map_err(|io_error| {
            FileSystemError::from(io_error)
        })
    } else {
        Ok(())
    }
}
//Check if file exists
fn exists(path: &Path) -> bool {
    path.exists()
}

//Get file's metadata
fn metadata(path: &Path) -> FileSystemResult<fs::Metadata> {
    path.metadata().map_err(|error| {
        FileSystemError::from(error)
    })
}

//Retrieve all file entries in the given directory (recursive).
fn read_dir(path: &Path) -> FileSystemResult<fs::ReadDir> {

    fs::read_dir(path).map_err(|io_error| {
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

    pub fn get_root_of(&self, root_dir: &RootDir) -> FileSystemResult<PathBuf> {
        let root_dir = self.game_directories.path(root_dir)?;
        Ok(root_dir.clone())
    }

    pub fn construct_path_from_root(&self, root_dir: &RootDir, path: &str) -> FileSystemResult<PathBuf> {
        let mut root_dir = self.get_root_of(root_dir)?;
        root_dir.push(path);
        Ok(root_dir)
    }

    pub fn get_absolute_path(&self, path: &Path) -> FileSystemResult<PathBuf> {
        get_absolute_path(path)
    }

    pub fn get_file_extension(&self, path: &Path) -> FileSystemResult<FileExtension> {
        get_extension(path)
    }

    //Open file at path to read
    pub fn open(&self, path: &Path) -> FileSystemResult<BufReader<File>> {
        open_as_bufreader(path)
    }

    //Open file at path for writing, truncates if file already exist
    pub fn create(&self, path: &Path) -> FileSystemResult<BufWriter<File>> {
        create_as_bufwriter(path)
    }

    //Open the file at path for appending, creating it if necessary
    pub fn append(&self, path: &Path) -> FileSystemResult<BufWriter<File>> {
        append_as_bufwriter(path)
    }

    //create directory at path
    pub fn mkdir(&self, path: &Path) -> FileSystemResult<()> {
        mkdir(path)
    }

    //remove a file
    pub fn rm(&self, path: &Path) -> FileSystemResult<()> {
        rm(path)
    }

    //remove file or directory and all its contents
    pub fn rmrf(&self, path: &Path) -> FileSystemResult<()> {
        rmrf(path)
    }

    //Check if file exists
    pub fn exists(&self, path: &PathBuf) -> bool {
        exists(path)
    }

    //Get file's metadata
    pub fn metadata(&self, path: &Path) -> FileSystemResult<fs::Metadata> {
        metadata(path)
    }

    //Retrieve all file entries in the given directory (recursive).
    pub fn read_dir(&self, path: &Path) -> FileSystemResult<fs::ReadDir> {
        read_dir(path)
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

        let current_dir_dir_test = filesystem.construct_path_from_root(&RootDir::WorkingDirectory, "dir_test").expect("Could not create current_dir_dir_test PathBuf");
        /*println!("{:?}", current_dir_dir_test);
        println!("{:?}", current_dir_dir_test.as_path());
        panic!();*/
        filesystem.mkdir(current_dir_dir_test.as_path()).expect("Could not create dir with current_dir_dir_test as path");
        assert!(filesystem.exists(&current_dir_dir_test));

        //user logs
        let user_log_dir_test = filesystem.construct_path_from_root(&RootDir::UserLogRoot, "log_dir_test").expect("Could not create user_log_dir_test");
        filesystem.mkdir(user_log_dir_test.as_path()).expect("Could not create dir with user_log_dir_test as path");
        assert!(filesystem.exists(&user_log_dir_test));

        let file_test = filesystem.construct_path_from_root(&RootDir::UserLogRoot, "log_dir_test/file_test.txt").expect("Could not create file_test.txt");
        let mut log_dir_bufwriter = filesystem.create(file_test.as_path()).expect("Could not create log_dir_test/file_test.txt");
        filesystem.write_all(&mut log_dir_bufwriter, b"text_test\n").expect("Couldn't add 'text test'");



        let async_dir = filesystem.construct_path_from_root(&RootDir::UserLogRoot, "async_dir").expect("Could not create async_dir");
        filesystem.mkdir(async_dir.as_path()).expect("Could not create dir with async_dir as path");
        assert!(filesystem.exists(&async_dir));
        //test async functionalities.
        let thread_pool = Configuration::new().build().expect("Could not create the thread pool.");
        let async_log_dir_test = filesystem.construct_path_from_root(&RootDir::UserLogRoot, "async_dir/async_log_dir_test.txt").expect("Could not create async_log_dir_test");
        {

            let mut log_bufwriter = filesystem.create(async_log_dir_test.as_path()).expect("Could not create the bufwriter");
            filesystem.async_write_all(&mut log_bufwriter, b"test_async_text_1\n", &thread_pool).expect("Could not write to file async_log_dir_test and write test_async_test_1 asynchronously");
            filesystem.async_write_all(&mut log_bufwriter, b"test_async_text_2\n", &thread_pool).expect("Could not write to file async_log_dir_test and write test_async_test_2 asynchronously");
            filesystem.async_write_all(&mut log_bufwriter, b"test_async_text_3\n", &thread_pool).expect("Could not write to file async_log_dir_test and write test_async_test_3 asynchronously");
        } //bufwriter dropped here, all the write calls will be executed.

        let mut bufreader_async = filesystem.open(async_log_dir_test.as_path()).expect("Could not create bufreader");
        let mut content = String::new();
        filesystem.async_read_to_string(&mut bufreader_async, &mut content, &thread_pool).expect("Could not read bufreader_async to string");
        let mut lines = content.lines();
        assert_eq!(lines.next(), Some("test_async_text_1"));
        assert_eq!(lines.next(), Some("test_async_text_2"));
        assert_eq!(lines.next(), Some("test_async_text_3"));
        assert_eq!(lines.next(), None);

        //Metadata
        let file_metadata = filesystem.metadata(async_log_dir_test.as_path()).expect("Couldn't get metadata");
        assert!(file_metadata.is_file());
        assert!(!file_metadata.is_dir());
        assert!(file_metadata.len() > 0);

        //remove
        filesystem.rm(async_log_dir_test.as_path()).expect("Couldn't delete the file : async_dir/async_log_dir_test.txt");
        assert!(!filesystem.exists(&async_log_dir_test));
        filesystem.rmrf(current_dir_dir_test.as_path()).expect("Couldn't delete dir");
        assert!(!filesystem.exists(&current_dir_dir_test));
    }


    #[test]
    fn filesystem_read_dir() {
        let filesystem = FileSystem::new(GameInfos::new("test_filesystem_blacksmith", "Malkaviel")).expect("Couldn't create FS");
        let src_dir = filesystem.construct_path_from_root(&RootDir::WorkingDirectory, "src").unwrap();
        let mut entries = filesystem.read_dir(src_dir.as_path()).unwrap();
        assert!(entries.next().is_some());
    }
}