

/// Represents the starting and ending points of a code location
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Span {
    starting_line: usize,
    ending_line: usize,
    
    starting_column: usize,
    ending_column: usize,
}

impl Span {
    /// Create a new Span object from the starting and ending points
    pub fn new(
        starting_line: usize,
        ending_line: usize,

        starting_column: usize,
        ending_column: usize,
    ) -> Span {
        Span {
            starting_line,
            ending_line,
            starting_column,
            ending_column
        }
    }

    /// Create a dummy span object 
    /// 
    /// Mainly used for convenience when testing
    pub fn dummy() -> Span {
        Span {
            starting_column: 0,
            starting_line: 0,
            ending_line: 0,
            ending_column: 0,
        }
    }
}
