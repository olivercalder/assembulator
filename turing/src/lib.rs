pub mod tape;

#[cfg(test)]
mod tests {
    use crate::tape::Tape;

    #[test]
    fn test_tape() {
        let input_string = String::from("hello");
        let modified_string = String::from("heelo");
        let truncated_string = String::from("helo");
        let mut t = Tape::new(&input_string);
        assert_eq!(t.output_tape(), input_string);
        t.right();
        t.right();
        assert_eq!(t.read(), Some('l'));
        assert_eq!(t.write(Some('e')), Some('l'));
        assert_eq!(t.output_tape(), modified_string);
        t.left();
        assert_eq!(t.write(None), Some('e'));
        assert_eq!(t.output_tape(), truncated_string);
    }
}
