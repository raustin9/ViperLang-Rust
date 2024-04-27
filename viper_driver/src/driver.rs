use std::{path::PathBuf, process::ExitCode};
pub use clap::Parser;
use viper_core::{buildsystem::BuildSystem, source::SourceFile};

/// Command line argument to the compiler for what file we want to compile
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Hash, clap::Parser)]
#[clap(
    name = "viper",
    about = "Viper programming language compiler",
    author = "raustin9"
)]
pub struct Argument {
    pub file: PathBuf,

    #[clap(long= "dump-syntax")]
    pub dump_syntax: bool,
}

/// Run the compiler on that argument
pub fn run(arg: Argument) -> ExitCode {
    let filepath = arg.file;
    let _builder = BuildSystem::new(filepath);



    // let file = SourceFile::new(filepath).unwrap();
    // println!("{file}");

    return ExitCode::SUCCESS;
}
