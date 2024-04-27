use std::{ffi::OsStr, path::PathBuf, sync::Arc};

use crate::source::{SourceFile, SourceModule};

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct BuildSystem {
    /// The specified path to compile
    path: PathBuf,

    /// List of pointers to source code files
    modules: Arc<[Arc<SourceModule>]>,
}

/*
                    Inspired by Cargo from Rust
├── Build.lock
├── Build.toml
├── src/
│   ├── main.viper
│   ├── module1/
│   │   ├── named-executable.viper
│   │   └── another-executable.viper
│   └── module2/
│       ├── named-executable.viper
└───────└── another-executable.viper
 */

impl BuildSystem {
    pub fn new(path: PathBuf) -> BuildSystem {
        let module = SourceModule::new(&path);


        BuildSystem {
            path,
            modules: Arc::new([Arc::new(module)])
        }
    }

    /// TODO: Build system for packages
    pub fn build_project(&self) {
        
    }

    /// Fully compile a source code file
    pub fn compile_file(&self, _file: Arc<SourceFile>) {
        
    }

}


/// Determine if a file path is a Viper source code file, other type of file, or a directory.
/// TODO: change from returning `bool` to returning an Enum for all valid file types
///
/// ## path: The specified path
/// ## returns `true` if it is a Viper source file and `false` otherwise
fn _path_is_valid_file(path: &PathBuf) -> bool {
    let extension = path.extension().and_then(OsStr::to_str);

    match extension {
        // We got a file extension
        Some(ref ext) => {
            match *ext {
                "viper" => {
                    return true;
                }
                _ => {
                    println!("Unknown file extension: {ext}");
                    return false;
                }
            }
        },
       
        // No file extension, path is probably for a directory
        None => {
            println!("Got directory: {}", path.as_path().display());
            return false;
        }
    };
}
