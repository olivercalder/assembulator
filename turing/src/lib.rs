pub mod automaton_components;
pub mod tape;
pub mod transition_table;

#[cfg(test)]
mod tests {
    use crate::{
        automaton_components::{MoveDirection, State, Symbol},
        tape::Tape,
        transition_table::TransitionTable,
    };

    #[test]
    fn test_symbol_to_option() {
        let non_empty = Symbol::from_option(Some('a'));
        let empty = Symbol::from_option(None);
        assert_eq!(non_empty, Symbol::Is('a'));
        assert_eq!(empty, Symbol::Blank);
        assert_eq!(non_empty.to_option(), Some('a'));
        assert_eq!(empty.to_option(), None);
    }

    #[test]
    fn test_tape() {
        let input_string = String::from("hello");
        let modified_string = String::from("heelo");
        let truncated_string = String::from("helo");
        let appended_string = String::from("helod");
        let inserted_string = String::from("heload");
        let mut t = Tape::new(&input_string);
        assert_eq!(t.output_tape(), input_string);
        t.left();
        assert_eq!(t.output_tape(), input_string);
        assert_eq!(t.read(), Symbol::Blank);
        t.right();
        t.right();
        t.right();
        assert_eq!(t.read(), Symbol::Is('l'));
        assert_eq!(t.write(Symbol::Is('e')), Symbol::Is('l'));
        assert_eq!(t.output_tape(), modified_string);
        t.left();
        assert_eq!(t.write(Symbol::Blank), Symbol::Is('e'));
        assert_eq!(t.output_tape(), truncated_string);
        t.right();
        t.right();
        t.right();
        t.right();
        t.right();
        assert_eq!(t.write(Symbol::Is('d')), Symbol::Blank);
        assert_eq!(t.output_tape(), appended_string);
        t.left();
        assert_eq!(t.output_tape(), appended_string);
        assert_eq!(t.write(Symbol::Is('a')), Symbol::Blank);
        assert_eq!(t.output_tape(), inserted_string);
    }

    #[test]
    fn test_transition_table() {
        let mut table = TransitionTable::new();
        let q0 = State::Start;
        let qh = State::Halt;
        let q1 = State::Other(String::from("q1"));
        table.add_transition(&q0, &q1, &[Symbol::Is('h')], false, Some(Symbol::Is('H')), MoveDirection::Right);
        table.add_transition(&q0, &q1, &[Symbol::Is('H')], false, None, MoveDirection::Right);
        table.add_transition(&q1, &qh, &[Symbol::Is('i')], false, None, MoveDirection::Stay);
        table.add_transition(&q1, &qh, &[Symbol::Is('I')], false, Some(Symbol::Is('i')), MoveDirection::Stay);
        table.add_transition(&q0, &qh, &[Symbol::Is('h'), Symbol::Is('H')], true, Some(Symbol::Blank), MoveDirection::Stay);
        let t1 = table.get_transition(&q0, Symbol::Is('h')).unwrap();
        assert_eq!(t1.get_next_state(), &q1);
        assert_eq!(t1.get_write_symbol(), Some(Symbol::Is('H')));
        assert_eq!(t1.get_move(), MoveDirection::Right);
        let t2 = table.get_transition(&q0, Symbol::Is('H')).unwrap();
        assert_eq!(t2.get_next_state(), &q1);
        assert_eq!(t2.get_write_symbol(), None);
        assert_eq!(t2.get_move(), MoveDirection::Right);
        let t3 = table.get_transition(&q0, Symbol::Is('A')).unwrap();
        assert_eq!(t3.get_next_state(), &qh);
        assert_eq!(t3.get_write_symbol(), Some(Symbol::Blank));
        assert_eq!(t3.get_move(), MoveDirection::Stay);
        let t4 = table.get_transition(&q0, Symbol::Blank).unwrap();
        assert_eq!(&t3, &t4);
        let t5 = table.get_transition(&q1, Symbol::Is('I')).unwrap();
        assert_eq!(t5.get_next_state(), &qh);
        assert_eq!(t5.get_write_symbol(), Some(Symbol::Is('i')));
        assert_eq!(t5.get_move(), MoveDirection::Stay);
        let t6 = table.get_transition(&q1, Symbol::Is('i')).unwrap();
        assert_eq!(t6.get_next_state(), &qh);
        assert_eq!(t6.get_write_symbol(), None);
        assert_eq!(t6.get_move(), MoveDirection::Stay);
        assert_eq!(table.get_transition(&q1, Symbol::Is('a')), None);
    }
}
