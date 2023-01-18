//! This module simulates an infinite tape with a head which can move, read, and write.

use std::collections::VecDeque;

/// An infinite tape with a head which can move, read, and write.
///
/// The tape is stored internally as a VecDeque which expands as the head moves beyond the cells
/// which had previously been populated.
///
/// Cells on the tape may be empty, and empty cells are stored as None in the internal array.
pub struct Tape {
    array: VecDeque<Option<char>>,
    position: usize,
}

impl Tape {
    /// Creates a new tape with contents loaded from the specified string slice.
    pub fn new(contents: &str) -> Self {
        Self {
            array: contents.chars().map(|x| Some(x)).collect::<VecDeque<Option<char>>>(),
            position: 0,
        }
    }

    /// Reads and returns the contents of the cell at the current position.
    pub fn read(&self) -> Option<char> {
        *self.array.get(self.position).unwrap()
    }

    /// Writes the given symbol to the cell at the current position, returning the previous
    /// contents of the cell.
    pub fn write(&mut self, symbol: Option<char>) -> Option<char> {
        let cell = self.array.get_mut(self.position).unwrap();
        let orig = *cell;
        *cell = symbol;
        orig
    }

    /// Moves the head position left one cell.
    pub fn left(&mut self) -> () {
        if self.position == 0 {
            self.array.push_front(None);
        } else {
            self.position -= 1;
        }
    }

    /// Moves the head position right one cell.
    pub fn right(&mut self) -> () {
        if self.position == self.array.len() - 1 {
            self.array.push_back(None);
        }
        self.position += 1;
    }

    /// Instructs the head to stay in its current position.
    ///
    /// This function does nothing and exists only for consistency.
    pub fn stay(&mut self) -> () {
        // do nothing
    }

    /// Exports the current contents of the tape into a String.
    ///
    /// Any empty cells are omitted entirely. That is, if two cells containing symbols are
    /// separated by some number of empty cells, then the symbols in those two full cells will be
    /// adjacent in the output string.
    pub fn output_tape(&self) -> String {
        self.array.iter().filter_map(|s| *s).collect::<String>()
    }
}
