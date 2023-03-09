//! This module provides the transitions and transition tables as found in Turing Machines.

use crate::automaton_components::{MoveDirection, State, Symbol};
use std::collections::BTreeMap;

#[derive(Debug, PartialEq, Eq)]
pub struct Transition {
    next_state: State,
    write: Option<Symbol>,
    direction: MoveDirection,
}

impl Transition {
    fn create(next_state: State, write: Option<Symbol>, direction: MoveDirection) -> Transition {
        Transition {
            next_state,
            write,
            direction,
        }
    }

    pub fn get_next_state(&self) -> &State {
        &self.next_state
    }

    pub fn get_write_symbol(&self) -> Option<Symbol> {
        self.write
    }

    pub fn get_move(&self) -> MoveDirection {
        self.direction
    }
}

struct TransitionTableEntry {
    transitions: Vec<Transition>,
    symbol_map: BTreeMap<Symbol, usize>, // map symbols to indices into transitions
    not_symbols: Option<Vec<Symbol>>,
    not_symbol_transition_index: usize,
}

impl TransitionTableEntry {
    fn new() -> TransitionTableEntry {
        TransitionTableEntry {
            transitions: Vec::new(),
            symbol_map: BTreeMap::new(),
            not_symbols: None,
            not_symbol_transition_index: 0,
        }
    }

    fn add_transition(
        &mut self,
        next_state: State,
        read: &[Symbol],
        not_read: bool,
        write: Option<Symbol>,
        direction: MoveDirection,
    ) -> &mut Self {
        self.transitions
            .push(Transition::create(next_state, write, direction));
        let index = self.transitions.len() - 1;
        if not_read == true {
            assert_eq!(
                self.not_symbols, None,
                "attempted to add multiple !symbol transitions"
            );
            self.not_symbols = Some(read.to_vec());
            self.not_symbol_transition_index = index;
        } else {
            for symbol in read {
                self.symbol_map.insert(*symbol, index);
            }
        }
        self
    }

    fn get_transition(&self, symbol_read: Symbol) -> Option<&Transition> {
        if let Some(index) = self.symbol_map.get(&symbol_read) {
            return Some(&self.transitions[*index]);
        }
        if let Some(slice) = &self.not_symbols {
            if slice.iter().find(|e| *e == &symbol_read) == None {
                return Some(&self.transitions[self.not_symbol_transition_index]);
            }
        }
        None
    }
}

pub struct TransitionTable {
    state_transitions: BTreeMap<State, TransitionTableEntry>,
}

impl TransitionTable {
    pub fn new() -> TransitionTable {
        TransitionTable {
            state_transitions: BTreeMap::new(),
        }
    }

    pub fn add_transition(
        &mut self,
        start: &State,
        end: &State,
        read_symbols: &[Symbol],
        not_read: bool,
        write_symbol: Option<Symbol>,
        direction: MoveDirection,
    ) -> &mut Self {
        self.state_transitions
            .entry(start.clone())
            .and_modify(|tte| {
                tte.add_transition(end.clone(), read_symbols, not_read, write_symbol, direction);
            })
            .or_insert_with(|| {
                let mut entry = TransitionTableEntry::new();
                entry.add_transition(end.clone(), read_symbols, not_read, write_symbol, direction);
                entry
            });
        self
    }

    pub fn get_transition(
        &self,
        current_state: &State,
        symbol_read: Symbol,
    ) -> Option<&Transition> {
        self.state_transitions
            .get(&current_state)
            .unwrap()
            .get_transition(symbol_read)
    }
}
