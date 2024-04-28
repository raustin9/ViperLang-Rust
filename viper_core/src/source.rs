use core::fmt;
use std::{fs, ops::Range, path::PathBuf, sync::Arc};
use crate::error::{ 
    IoError, VError
};
use colored::*;

/// Represents the location of a lexeme within a source code file
#[derive(Clone, Debug, Copy, Default, PartialEq, Eq)]
pub struct SourceLocation {
    pub line: usize,
    pub column: usize,
}

/// Represents a source code file
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct SourceFile {
    source_code: Arc<[u8]>,
    // source_code: String,
    source_name: PathBuf,
}


/// Represents a module of source files
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct SourceModule {
    path: PathBuf,
    name: String,
    // files: Arc<[Arc<SourceFile>]>
    files: Vec<Arc<SourceFile>>
}


impl SourceModule {
    /// Create a new module from the specified path
    /// TODO: figure out good way of naming modules internally
    ///       ex: `mod1/foo/bar` how should that be named?
    pub fn new(path: &PathBuf) -> SourceModule {
        let mod_name = path.to_str()
            .expect("Unable to get name of module from path!")
            .to_owned();

        let module = SourceModule {
            path: path.clone(),
            name: mod_name,
            files: SourceModule::find_files(&path),
            // files: Vec::new()
        };

        return module;
    }

    /// Get a reference to the list of source code files 
    /// for this module
    pub fn files(&self) -> &Vec<Arc<SourceFile>> {
        return &self.files;
    }

    /// Go through the directory for this module and 
    /// find all the source code files
    pub fn find_files(path: &PathBuf) -> Vec<Arc<SourceFile>> {
        let paths = fs::read_dir(&path).unwrap();
        let mut files = Vec::new();
        // Create pointers to each source code file
        // NOTE: This assumes that every filepath in a directory is 
        //       a source code file. This is not going to hold for 
        //       long because we want submodules with subdirectories
        for path in paths {
            println!("Path: {}", 
                path.as_ref()
                    .unwrap()
                    .path()
                    .display());
            
            let f = Arc::new(SourceFile::new(path.unwrap().path()).unwrap());
            files.push(f);
        }

        return files;
    }
}

impl fmt::Display for SourceModule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Module name: {}. Number of files: {}"
            , self.name
            , self.files.len()
        )
    }
}


impl SourceFile {
    /// Create a new source code file from the specified path
    pub fn new(path: PathBuf) -> Result<SourceFile, VError> {
        let contents = fs::read_to_string(path.clone());

        match contents {
            Ok(content) => {
                return Ok(SourceFile {
                    source_code: Arc::from(content.as_bytes()),
                    source_name: path,
                });
            },
            Err(_) => {
                return Err(IoError::new("Unable to read from file!"));
            }
        };
    }

    /// Get a reference to the source code of the file
    pub fn code(&self) -> &Arc<[u8]> {
        return &self.source_code;
    }

    /// Get the PathBuf or filepath to the source code file
    pub fn name(&self) -> &PathBuf {
        return &self.source_name;
    }
}

impl fmt::Display for SourceFile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Name: {}\nContent:\n{}", self.source_name.to_path_buf().display(), std::str::from_utf8(&self.source_code).unwrap().green())
    }
}

fn _get_line_byte_positions(text: &str) -> Vec<Range<usize>> {
    let mut current_position = 0;
    let mut results = Vec::new();

    let mut skip = false;

    for (byte, char) in text.char_indices() {
        if skip {
            skip = false;
            continue;
        }

        // ordinary lf
        if char == '\n' {
            #[allow(clippy::range_plus_one)]
            results.push(current_position..byte + 1);

            current_position = byte + 1;
        }

        // crlf
        if char == '\r' {
            if text.as_bytes().get(byte + 1) == Some(&b'\n') {
                #[allow(clippy::range_plus_one)]
                results.push(current_position..byte + 2);

                current_position = byte + 2;

                skip = true;
            } else {
                #[allow(clippy::range_plus_one)]
                results.push(current_position..byte + 1);

                current_position = byte + 1;
            }
        }
    }

    results.push(current_position..text.len());

    results
}
