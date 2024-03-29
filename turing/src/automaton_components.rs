//! This module provides the abstraction for basic components of Turing machines and other
//! automata.

/// Symbols are chars which may be either empty or non-empty.
#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum Symbol {
    Is(char),
    Blank,
}

impl Symbol {
    /// Converts a Option<char> to a symbol, where None corresponds to Symbol::Blank.
    pub fn from_option(option: Option<char>) -> Symbol {
        match option {
            Some(x) => Symbol::Is(x),
            None => Symbol::Blank,
        }
    }

    /// Converts a symbol to its corresponding Option<char>, where Blank cells are None.
    pub fn to_option(&self) -> Option<char> {
        match self {
            Symbol::Is(x) => Some(*x),
            Symbol::Blank => None,
        }
    }
}

/// The read/write head of an automaton may move either left, right, or stay where it began.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MoveDirection {
    Left,
    Right,
    Stay,
}

/// The states available to be used in automata, including special states.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum State {
    Start,
    Accept,
    Reject,
    Halt,
    Other(String),
}
