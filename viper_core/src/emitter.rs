use crate::error::{ViperError, ViperWarning};


/// A lot of this is inspired from Leo Lang!
/// thanks https://github.com/AleoHQ/leo

/// Types that are sinks for compiler errors
pub trait Emitter {
    /// Emit an error
    fn emit_err(&mut self, err: ViperError);

    /// Tracks the last emitted error
    fn last_emitted_error_code(&self) -> Option<i32>;

    /// Emit a warning
    fn emit_warning(&mut self, warning: ViperWarning);
}


/// Standard emitter that emits to StdErr
pub struct StdEmitter {
    last_error_code: Option<i32>,
}

impl Emitter for StdEmitter {
    fn emit_err(&mut self, err: ViperError) {
        self.last_error_code = Some(err.error_code());

        eprintln!("{err}");
    }

    fn last_emitted_error_code(&self) -> Option<i32> {
        self.last_error_code
    }

    fn emit_warning(&mut self, warning: ViperWarning) {
        eprintln!("{warning}")
    }
}
