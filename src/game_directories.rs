// Copyright 2017-2018 Maskerad Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use std::collections::HashMap;

use std::path::{Path, PathBuf};
use std::env;
use game_infos::GameInfos;
use filesystem_error::{FileSystemError, FileSystemResult};

//Enum used to specify the 'root' directory from where to write/delete/open dir/files
#[derive(Debug, Eq, PartialEq, Hash)]
pub enum RootDir {
    WorkingDirectory,
    UserDataRoot,
    UserConfigRoot,
    UserEngineConfigurationRoot,
    UserLogRoot,
    UserSaveRoot,
}

#[derive(Debug)]
pub struct GameDirectories(HashMap<RootDir, PathBuf>);

impl GameDirectories {
    pub fn new(game_infos: GameInfos) -> FileSystemResult<Self> {
        let user_config = if cfg!(target_os = "windows") {
            let appdata = env::var("APPDATA")?;
            PathBuf::from(format!(
                "{}\'{}\'{}",
                appdata.as_str(),
                game_infos.author().as_str(),
                game_infos.game_name().as_str()
            ))
        } else if cfg!(target_os = "macos") {
            unimplemented!();
        } else {
            let home = env::var("HOME")?;
            PathBuf::from(format!(
                "{}/.config/{}",
                home.as_str(),
                game_infos.game_name().as_str()
            ))
        };

        let user_data = if cfg!(target_os = "windows") {
            let appdata = env::var("APPDATA")?;
            PathBuf::from(format!(
                "{}\'{}\'{}",
                appdata.as_str(),
                game_infos.author().as_str(),
                game_infos.game_name().as_str()
            ))
        } else if cfg!(target_os = "macos") {
            unimplemented!();
        } else {
            let home = env::var("HOME")?;
            PathBuf::from(format!(
                "{}/.local/share/{}",
                home.as_str(),
                game_infos.game_name().as_str()
            ))
        };

        let mut logs = user_config.clone();
        logs.push("maskerad_logs");
        let mut engine_config = user_config.clone();
        engine_config.push("maskerad_configuration");
        let mut saves = user_data.clone();
        saves.push("game_saves");
        let current = env::current_dir()?;

        let mut directories = HashMap::with_capacity(6);
        directories.insert(RootDir::WorkingDirectory, current);
        directories.insert(RootDir::UserDataRoot, user_data);
        directories.insert(RootDir::UserConfigRoot, user_config);
        directories.insert(RootDir::UserEngineConfigurationRoot, engine_config);
        directories.insert(RootDir::UserLogRoot, logs);
        directories.insert(RootDir::UserSaveRoot, saves);

        Ok(GameDirectories(directories))
    }

    pub fn path(&self, root_dir: &RootDir) -> FileSystemResult<&Path> {
        match self.0.get(root_dir) {
            Some(pathbuf_ref) => Ok(pathbuf_ref.as_path()),
            None => Err(FileSystemError::GameDirectoryError(format!(
                "The associated path for {:?} could not be found !",
                root_dir
            ))),
        }
    }

    pub fn construct_path_from_root(
        &self,
        root_dir: &RootDir,
        path: &str,
    ) -> FileSystemResult<PathBuf> {
        let mut root_dir = self.path(root_dir)?.to_path_buf();
        root_dir.push(path);
        Ok(root_dir)
    }
}
