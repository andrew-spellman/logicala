use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

pub struct RepeatsNoWhiteSpace {
    reader: BufReader<File>,
    pub current_line: String,
    current_chars: Vec<char>,
    pub line_number: usize,
    pub char_index: usize,
}

impl RepeatsNoWhiteSpace {
    pub fn new(file: File) -> Self {
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
    use std::fs::{self, File, OpenOptions};
    use std::io::{Read, Seek, Write};

    fn test_file_from_str(file_name: &str, s: &str) -> File {
        _ = fs::remove_file(&file_name);
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(file_name)
            .unwrap();
        file.write_all(s.as_bytes()).unwrap();
        file.seek(std::io::SeekFrom::Start(0)).unwrap();
        return file;
    }

    #[test]
    fn test_file() {
        let path = "/tmp/foo0";
        let mut file = test_file_from_str(&path, "bar");
        let mut buffer = String::new();
        file.read_to_string(&mut buffer).unwrap();
        assert_ne!(&mut buffer, "");
    }

    #[test]
    fn base_case() {
        let path = "/tmp/foo1";
        let file = test_file_from_str(&path, "bar");
        let mut stream = RepeatsNoWhiteSpace::new(file);
        assert_eq!(stream.get(), Some('b'));
        assert_eq!(stream.next(), Some('b'));

        assert_eq!(stream.get(), Some('a'));
        assert_eq!(stream.next(), Some('a'));

        assert_eq!(stream.get(), Some('r'));
        assert_eq!(stream.next(), Some('r'));

        assert_eq!(stream.get(), None);
        assert_eq!(stream.next(), None);
        assert_eq!(stream.next(), None);
    }

    #[test]
    fn spaces() {
        let path = "/tmp/foo2";
        let file = test_file_from_str(&path, "b   ar");
        let mut stream = RepeatsNoWhiteSpace::new(file);
        assert_eq!(stream.get(), Some('b'));
        assert_eq!(stream.next(), Some('b'));

        assert_eq!(stream.get(), Some(' '));
        assert_eq!(stream.next(), Some(' '));

        assert_eq!(stream.get(), Some('a'));
        assert_eq!(stream.next(), Some('a'));

        assert_eq!(stream.get(), Some('r'));
        assert_eq!(stream.next(), Some('r'));

        assert_eq!(stream.get(), None);
        assert_eq!(stream.next(), None);
        assert_eq!(stream.next(), None);
    }

    #[test]
    fn new_lines() {
        let path = "/tmp/foo3";
        let file = test_file_from_str(&path, "b\n\n\nar");
        let mut stream = RepeatsNoWhiteSpace::new(file);
        assert_eq!(stream.get(), Some('b'));
        assert_eq!(stream.next(), Some('b'));

        assert_eq!(stream.get(), Some('\n'));
        assert_eq!(stream.next(), Some('\n'));

        assert_eq!(stream.get(), Some('a'));
        assert_eq!(stream.next(), Some('a'));

        assert_eq!(stream.get(), Some('r'));
        assert_eq!(stream.next(), Some('r'));

        assert_eq!(stream.get(), None);
        assert_eq!(stream.next(), None);
        assert_eq!(stream.next(), None);
    }
}
