use std::env;
use std::error::Error;
use std::path::Path;

/// Manager for a list of "search paths" on the filesystem, for the purpose of
/// resolving the absolute path of data files that have several possible
/// locations. See the documentation of 'find' for more information on the
/// default search paths.
#[derive(Debug)]
pub struct PathManager {
    search: Vec<String>,
}

impl PathManager {
    // TODO: Implement the following additional search paths:
    // - The steam paths on GNU/Linux.
    // - The "standard" paths for OSX (See G_AddSearchPaths)
    // - The "standard" paths for Windows (See G_AddSearchPaths)
    // - Those specified on the command-line.
    //   - See the CommandPaths and CommandGrps globals in common.cpp
    // - The "app dir" on OSX.
    // - PROPERLY add the CWD.
    // - $HOME/apps/rebuild/ (?)
    // - $HOME/.config/rebuild/
    // - Those specified via DUKE3DGRP environment variable.
    //   - See JBF 20031220

    /// Create a new PathManager and initialize the search path list with
    /// acceptable defaults. See the documentation of 'find' for more
    /// information on the default search paths.
    pub fn new() -> PathManager {
        let mut result = PathManager { search: Vec::new() };

        let directories = vec![
            ".",
            "/usr/share/games/jfduke3d",
            "/usr/local/share/games/jfduke3d",
            "/usr/share/games/eduke32",
            "/usr/local/share/games/eduke32",
            "/usr/share/games/rebuild",
            "/usr/local/share/games/rebuild",
        ];

        for directory in directories.iter() {
            result.add_path(directory).ok();
        }

        let directories = vec![
            "$HOME/.rebuild",
        ];

        if let Some(home) = env::home_dir() {
            if let Some(home) = home.to_str() {
                for path in directories.iter() {
                    let path = String::from(*path).replace("$HOME", home);
                    result.add_path(&path).ok();
                }
            }
        }

        result
    }

    /// Add an additional path for searching.
    ///
    /// # Errors
    ///
    /// A return value of 'Err' indicates that the given path did not exist.
    pub fn add_path(&mut self, path: &str) -> Result<(), Box<Error>> {
        if Path::new(&path).exists() {
            self.search.push(String::from(path));
        } else {
            bail!("Path does not exist");
        }

        Ok(())
    }

    /// Go through the search path list in order and return the first existing
    /// path with the the given name, or None if no such file was found. The
    /// default search path order is:
    ///
    /// - ".",
    /// - "/usr/local/share/games/rebuild",
    /// - "/usr/share/games/rebuild",
    /// - "/usr/local/share/games/eduke32",
    /// - "/usr/share/games/eduke32",
    /// - "/usr/local/share/games/jfduke3d",
    /// - "/usr/share/games/jfduke3d",
    /// - "$HOME/.rebuild",
    ///
    /// This is followed by any additional paths in the search path list tha`t
    /// were added by 'add_path'.
    pub fn find(&self, name: &str) -> Option<String> {
        for directory in self.search.iter() {
            let path = format!("{}/{}", directory, name);

            if Path::new(&path).exists() {
                return Some(path);
            }
        }

        None
    }
}
