//! This module provides the abstraction for basic components of Turing machines and other
//! automata.

/// Symbols are chars which may be either empty or non-empty.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Symbol {
    Is(char),
    Empty,
}

impl Symbol {
    /// Converts a Option<char> to a symbol, where None corresponds to Symbol::Empty.
    pub fn from_option(option: Option<char>) -> Symbol {
        match option {
            Some(x) => Symbol::Is(x),
            None => Symbol::Empty,
        }
    }

    /// Converts a symbol to its corresponding Option<char>, where Empty cells are None.
    pub fn to_option(&self) -> Option<char> {
        match self {
            Symbol::Is(x) => Some(*x),
            Symbol::Empty => None,
        }
    }
}

/// The read/write head of an automaton may move either left, right, or stay where it began.
pub enum MoveDirection {
    Left,
    Right,
    Stay,
}
