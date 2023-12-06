use std::io::{BufRead, BufReader, Read};

pub struct RepeatsNoWhiteSpace {
    reader: BufReader<Box<dyn Read>>,
    pub current_line: String,
    current_chars: Vec<char>,
    pub line_number: usize,
    pub char_index: usize,
}

impl RepeatsNoWhiteSpace {
    pub fn new(file: Box<dyn Read>) -> Self {
        let reader = BufReader::new(file);
        let current_line = String::new();
        let current_chars = vec![];
        let mut new_self = Self {
            reader,
            current_line,
            current_chars,
            line_number: 0,
            char_index: 0,
        };
        new_self.read_line();
        new_self
    }

    pub fn get(&self) -> Option<char> {
        match self.current_chars.get(self.char_index) {
            None => None,
            Some(c) => Some(*c),
        }
    }

    fn take(&mut self) -> Option<char> {
        let taken = match self.get() {
            None => return None,
            Some(c) => Some(c),
        };
        self.char_index += 1;
        taken
    }

    fn read_line(&mut self) -> usize {
        self.current_line.clear();
        let bytes_read = self.reader.read_line(&mut self.current_line).unwrap();
        self.current_chars = self.current_line.chars().collect();
        self.line_number += 1;
        self.char_index = 0;
        bytes_read
    }
}

impl Iterator for RepeatsNoWhiteSpace {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        match self.take() {
            None => return None,
            Some(' ') => loop {
                match self.get() {
                    None => return None,
                    Some(' ') => self.char_index += 1,
                    _ => return Some(' '),
                }
            },
            Some('\n') => loop {
                match self.get() {
                    None => match self.read_line() {
                        0 => return None,
                        _ => (),
                    },
                    Some('\n') => self.char_index += 1,
                    _ => return Some('\n'),
                }
            },
            Some(c) => Some(c),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::file_from_str;

    #[test]
    fn base_case() {
        let file = file_from_str("bar");
        let mut reader = RepeatsNoWhiteSpace::new(Box::new(file));
        assert_eq!(reader.get(), Some('b'));
        assert_eq!(reader.next(), Some('b'));

        assert_eq!(reader.get(), Some('a'));
        assert_eq!(reader.next(), Some('a'));

        assert_eq!(reader.get(), Some('r'));
        assert_eq!(reader.next(), Some('r'));

        assert_eq!(reader.get(), None);
        assert_eq!(reader.next(), None);
        assert_eq!(reader.next(), None);
    }

    #[test]
    fn spaces() {
        let file = file_from_str("b   ar");
        let mut reader = RepeatsNoWhiteSpace::new(Box::new(file));
        assert_eq!(reader.get(), Some('b'));
        assert_eq!(reader.next(), Some('b'));

        assert_eq!(reader.get(), Some(' '));
        assert_eq!(reader.next(), Some(' '));

        assert_eq!(reader.get(), Some('a'));
        assert_eq!(reader.next(), Some('a'));

        assert_eq!(reader.get(), Some('r'));
        assert_eq!(reader.next(), Some('r'));

        assert_eq!(reader.get(), None);
        assert_eq!(reader.next(), None);
        assert_eq!(reader.next(), None);
    }

    #[test]
    fn new_lines() {
        let file = file_from_str("b\n\n\nar");
        let mut reader = RepeatsNoWhiteSpace::new(Box::new(file));
        assert_eq!(reader.get(), Some('b'));
        assert_eq!(reader.next(), Some('b'));

        assert_eq!(reader.get(), Some('\n'));
        assert_eq!(reader.next(), Some('\n'));

        assert_eq!(reader.get(), Some('a'));
        assert_eq!(reader.next(), Some('a'));

        assert_eq!(reader.get(), Some('r'));
        assert_eq!(reader.next(), Some('r'));

        assert_eq!(reader.get(), None);
        assert_eq!(reader.next(), None);
        assert_eq!(reader.next(), None);
    }
}
