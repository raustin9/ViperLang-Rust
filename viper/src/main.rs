use std::process::ExitCode;
use viper_driver::driver::{self, Argument, Parser};

fn main() -> ExitCode {
    let arg = Argument::parse();
    
    return driver::run(arg);
}
