pub mod automaton_components;
pub mod tape;

#[cfg(test)]
mod tests {
    use crate::automaton_components::Symbol;
    use crate::tape::Tape;

    #[test]
    fn test_symbol_to_option() {
        let non_empty = Symbol::from_option(Some('a'));
        let empty = Symbol::from_option(None);
        assert_eq!(non_empty, Symbol::Is('a'));
        assert_eq!(empty, Symbol::Empty);
        assert_eq!(non_empty.to_option(), Some('a'));
        assert_eq!(empty.to_option(), None);
    }

    #[test]
    fn test_tape() {
        let input_string = String::from("hello");
        let modified_string = String::from("heelo");
        let truncated_string = String::from("helo");
        let mut t = Tape::new(&input_string);
        assert_eq!(t.output_tape(), input_string);
        t.right();
        t.right();
        assert_eq!(t.read(), Symbol::Is('l'));
        assert_eq!(t.write(Symbol::Is('e')), Symbol::Is('l'));
        assert_eq!(t.output_tape(), modified_string);
        t.left();
        assert_eq!(t.write(Symbol::Empty), Symbol::Is('e'));
        assert_eq!(t.output_tape(), truncated_string);
    }
}
