use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct InputStream {
    reader: BufReader<File>,
    current_line: String,
    line_number: usize,
    char_index: usize,
}

impl InputStream {
    pub fn new(file: File) -> Self {
        let mut reader = BufReader::new(file);
        let mut current_line = String::new();
        reader.read_line(&mut current_line).unwrap();
        Self {
            reader,
            current_line,
            line_number: 1,
            char_index: 0,
        }
    }

    pub fn peek(&self) -> Option<char> {
        self.current_line.chars().nth(self.char_index)
    }

    // returns false if at end of file
    pub fn advance(&mut self) -> bool {
        let current_char = self.peek();
        self.char_index += 1;
        match current_char {
            None => return false,
            Some(' ') => loop {
                match self.peek() {
                    None => return false,
                    Some(' ') => self.char_index += 1,
                    _ => break,
                }
            },
            Some('\n') => loop {
                match self.peek() {
                    None => {
                        self.current_line.clear();
                        match self.reader.read_line(&mut self.current_line).unwrap() {
                            0 => return false,
                            _ => (),
                        }
                        self.line_number += 1;
                        self.char_index = 0;
                    }
                    Some('\n') => self.char_index += 1,
                    _ => break,
                }
            },
            _ => (),
        }
        if self.peek().is_none() {
            return false;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{File, OpenOptions};
    use std::io::{Read, Seek, Write};

    fn test_file_from_str(file_name: &str, s: &str) -> File {
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
        let mut file = test_file_from_str("/tmp/foo0", "bar");
        let mut buffer = String::new();
        file.read_to_string(&mut buffer).unwrap();
        assert_ne!(&mut buffer, "");
    }

    #[test]
    fn base_case() {
        let file = test_file_from_str("/tmp/foo1", "bar");
        let mut stream = InputStream::new(file);
        assert_eq!(stream.peek(), Some('b'));
        assert_eq!(stream.advance(), true);
        assert_eq!(stream.peek(), Some('a'));
        assert_eq!(stream.advance(), true);
        assert_eq!(stream.peek(), Some('r'));
        assert_eq!(stream.advance(), false);
        assert_eq!(stream.peek(), None);
        assert_eq!(stream.advance(), false);
        assert_eq!(stream.advance(), false);
    }

    #[test]
    fn spaces() {
        let file = test_file_from_str("/tmp/foo2", "b   ar");
        let mut stream = InputStream::new(file);
        assert_eq!(stream.peek(), Some('b'));
        assert_eq!(stream.advance(), true);
        assert_eq!(stream.peek(), Some(' '));
        assert_eq!(stream.advance(), true);
        assert_eq!(stream.peek(), Some('a'));
        assert_eq!(stream.advance(), true);
        assert_eq!(stream.peek(), Some('r'));
        assert_eq!(stream.advance(), false);
        assert_eq!(stream.peek(), None);
        assert_eq!(stream.advance(), false);
        assert_eq!(stream.advance(), false);
    }

    #[test]
    fn new_lines() {
        let file = test_file_from_str("/tmp/foo3", "b\n\n\nar");
        let mut stream = InputStream::new(file);
        assert_eq!(stream.peek(), Some('b'));
        assert_eq!(stream.advance(), true);
        assert_eq!(stream.peek(), Some('\n'));
        assert_eq!(stream.advance(), true);
        assert_eq!(stream.peek(), Some('a'));
        assert_eq!(stream.advance(), true);
        assert_eq!(stream.peek(), Some('r'));
        assert_eq!(stream.advance(), false);
        assert_eq!(stream.peek(), None);
        assert_eq!(stream.advance(), false);
        assert_eq!(stream.advance(), false);
    }
}
