use std::{ffi::OsStr, path::PathBuf, sync::Arc};
use colored::*;

use crate::source::{SourceFile, SourceModule};

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct BuildSystem {
    /// The specified path to compile
    path: PathBuf,

    /// List of pointers to source code files
    modules: Vec<Arc<SourceModule>>,
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
        let module = Arc::new(SourceModule::new(&path));


        BuildSystem {
            path,
            modules: vec![module],
        }
    }

    /// TODO: Build system for packages
    pub fn build_project(&self) {
        for module in &self.modules {
            println!("{}", format!("Compiling: {module}").bright_cyan());
            self.build_module(module);
        }
    }

    /// Build a module from the source code files in it
    pub fn build_module(&self, module: &Arc<SourceModule>) {
        for file in module.files() {
            self.compile_file(file);
        }
    }

    /// Fully compile a source code file
    pub fn compile_file(&self, file: &Arc<SourceFile>) {
        println!(
            "{}",
            format!(" -- Compiling file: {}", 
                file.name().as_path().display()
            )
            .bright_green()
        );
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
