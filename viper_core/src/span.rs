use std::ops::Add;

/// Represents the starting and ending points of a code location
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Span {
    start: CodeLocation,
    end: CodeLocation,
}

/// Represents a location within some source code
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd)]
pub struct CodeLocation {
    line: usize,
    column: usize,
}

impl CodeLocation {
    /// Create CodeLocation from the line and column
    fn new(line: usize, column: usize) -> CodeLocation {
        CodeLocation {
            line,
            column,
        }
    }

    /// Create a new dummy location
    fn new_dummy() -> CodeLocation {
        CodeLocation {
            line: 0,
            column: 0,
        }
    }
}

impl Ord for CodeLocation {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.line, self.column).cmp(&(other.line, other.column))
    }

    /// Return which location appeared before the other
    fn min(self, other: Self) -> Self
        where
            Self: Sized, {
        if self.line == other.line {
            if self.column < other.column {
                return self;
            }
            return other;
        }

        if self.line < other.line {
            return self;
        }
        return other;
    }
    
    /// Return which location appears after the other
    fn max(self, other: Self) -> Self
        where
            Self: Sized, {
        if self.line == other.line {
            if self.column > other.column {
                return self;
            }
            return other;
        }

        if self.line > other.line {
            return self;
        }
        return other;
    }
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
            start: CodeLocation::new(starting_line, starting_column),
            end: CodeLocation::new(ending_line, ending_column),
        }
    }

    /// Create a dummy span object 
    /// 
    /// Mainly used for convenience when testing
    pub fn dummy() -> Span {
        Span {
            start: CodeLocation::new_dummy(),
            end: CodeLocation::new_dummy(),
        }
    }
}

/// Add two spanning objects together to create a span that 
/// contains the entire region within both
impl Add for Span {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Span {
            start: std::cmp::min(self.start, rhs.start),
            end: std::cmp::max(self.end, rhs.end),
        }
    }
}
