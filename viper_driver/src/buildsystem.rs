use std::{ffi::OsStr, fs, path::PathBuf, sync::Arc};
use colored::*;
use viper_core::{source::{SourceFile, SourceModule}, token::Token};

use viper_lexer::lexer::Lexer;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct BuildSystem {
    /// The specified path to compile
    path: PathBuf,

    /// List of pointers to source code files
    modules: Option<Vec<Arc<SourceModule>>>,
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
    /// Configure the build system from the specified input.
    /// The input can either be a path to a file or a path 
    /// to a directory. 
    /// If a path to a file is specified, we will just compile that file,
    /// it a path to a directory is specified, we will build the module
    /// as a whole.
    pub fn new(path: PathBuf) -> BuildSystem {
        match path_is_directory(&path) {
            true => {
                println!(
                    "{}",
                    format!("Viper found module '{}'", path.as_path().display()).bright_cyan()
                );
                let module = Arc::new(SourceModule::new(&path));
                BuildSystem {
                    path,
                    modules: Some(vec![module]),
                }
            }

            false => {
                println!(
                    "{}",
                    format!("Viper found file '{}'", path.as_path().display()).bright_blue()
                );
                BuildSystem {
                    path,
                    modules: None,
                }
            }
        }


    }

    /// TODO: Build system for packages
    pub fn build_project(&self) {
        match self.modules {
            Some(ref modules) => {
                for module in modules {
                    println!("{}", format!("Compiling: {module}").bright_cyan());
                    self.build_module(&module);
                }
            }

            None => {
                let file = SourceFile::new(self.path.clone());
                match file {
                    Ok(source_file) => {
                        let file_ptr = Arc::new(source_file);
                        self.compile_file(&file_ptr);
                    }

                    Err(err) => {
                        println!("Error before compiling file: {}", err);
                    }
                }
            }
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

        let mut lexer = Lexer::new(file);
        lexer.print_test();
        let mut tok = lexer.next_token();
        while tok != Token::EOF {
            println!(
                "{}",
                format!("Test Token1: {}", tok).bright_white()
            );
            tok = lexer.next_token();
        }
    }
}

/// Determine if a specified path points to a directory 
/// or a file
fn path_is_directory(path: &PathBuf) -> bool {
    match fs::metadata(path) {
        Ok(ref md) => {
            if md.is_dir() {
                return true;
            }
            return false;
        }
        Err(_) => {
            println!("Error finding metadata for path: {}", path.as_path().display());
            return false;
        }
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
