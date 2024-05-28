use core::fmt;
use std::{cell::RefCell, fs, path::PathBuf, sync::Arc};
use colored::*;

use crate::{error::ViperError, scope::Scope};

/// Represents the location of a lexeme within a source code file
#[derive(Clone, Debug, Copy, Default, PartialEq, Eq)]
pub struct SourceLocation {
    pub line: usize,
    pub column: usize,
}

/// Represents a source code file
#[derive(Clone, Debug, PartialEq)]
pub struct SourceFile {
    // source_code: Box<[u8]>,
    source_code: Box<str>,
    // source_code: String,
    source_name: PathBuf,

    /// File-wide scope. Symbols declared at the file level
    /// are available here. 
    scope: Arc<RefCell<Scope>>,
}


/// Represents a module of source files
#[derive(Clone, Debug, PartialEq)]
pub struct SourceModule {
    path: PathBuf,
    name: String,

    /// Module-wide scope. Symbols that are meant to be public 
    /// across the entire module are declared at this scope
    scope: Arc<RefCell<Scope>>,
    // files: Box<[Box<SourceFile>]>
    files: Vec<Arc<SourceFile>>,
}


impl SourceModule {
    /// Create a new module from the specified path
    /// TODO: figure out good way of naming modules internally
    ///       ex: `mod1/foo/bar` how should that be named?
    pub fn new(path: &PathBuf) -> SourceModule {
        let mod_name = path.to_str()
            .expect("Unable to get name of module from path!")
            .to_owned();
        let scope = Arc::from(RefCell::new(Scope::new(None)));

        let module = SourceModule {
            path: path.clone(),
            name: mod_name,
            scope: scope.clone(),
            files: SourceModule::find_files(path, &scope),
        };

        return module;
    }

    /// Create a new dummy module that stores no real information
    pub fn new_dummy() -> SourceModule {
        SourceModule {
            path: PathBuf::from(""),
            name: "DUMMY MODULE".into(),
            scope: Arc::from(RefCell::new(Scope::new(None))),
            files: Vec::new(),
        }
    }

    /// Get a reference to the list of source code files 
    /// for this module
    pub fn files(&self) -> &Vec<Arc<SourceFile>> {
        return &self.files;
    }

    /// Find all the source code files within a module
    pub fn find_files(path: &PathBuf, parent_scope: &Arc<RefCell<Scope>>) -> Vec<Arc<SourceFile>> {
        let paths = fs::read_dir(&path).unwrap();
        let mut files = Vec::new();
        
        // Create pointers to each source code file
        // NOTE: This assumes that every filepath in a directory is 
        //       a source code file. This is not going to hold for 
        //       long because we want submodules with subdirectories
        for path in paths {
//            println!("Path: {}", 
//                path.as_ref()
//                    .unwrap()
//                    .path()
//                    .display());
            
            let f = Arc::new(SourceFile::new(path.unwrap().path(), parent_scope).unwrap());
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
    pub fn new(path: PathBuf, parent_scope: &Arc<RefCell<Scope>>) -> Result<SourceFile, ViperError> {
        let contents = fs::read_to_string(path.clone());

        match contents {
            Ok(content) => {
                return Ok(SourceFile {
                    source_code: Box::from(content),
                    scope: Arc::from(RefCell::new(Scope::new(Some(parent_scope.clone())))), // create this file's
                                                                              // scope and set it's
                                                                              // parent scope
                    source_name: path,
                });
            },
            Err(_) => {
                return Err(ViperError::IoError);
                // return Err(IoError::new("Unable to read from file!"));
            }
        };
    }

    pub fn new_dummy(content: &'static str, name: &'static str) -> SourceFile {
        return SourceFile {
            source_code: Box::from(content),
            source_name: PathBuf::from(name),
            scope: Arc::from(RefCell::new(Scope::new(None))),
        };
    }

    /// Return a pointer to the file's scope
    pub fn scope(&self) -> Arc<RefCell<Scope>> {
        self.scope.clone()
    }

    /// Get a reference to the source code of the file
    pub fn code(&self) -> &Box<str> {
        return &self.source_code;
    }

    /// Get the PathBuf or filepath to the source code file
    pub fn name(&self) -> &PathBuf {
        return &self.source_name;
    }
}

impl fmt::Display for SourceFile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Name: {}\nContent:\n{}", self.source_name.to_path_buf().display(), self.source_code.green())
    }
}
