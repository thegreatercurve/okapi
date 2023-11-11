#[derive(Debug)]
pub struct Lexer<'a> {
    input: &'a str,
    start_position: usize,
    read_position: usize,
    ch: char,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut lexer = Self {
            input: &input,
            start_position: 0,
            read_position: 0,
            ch: 0 as char,
        };

        lexer.read_char();

        return lexer;
    }

    pub fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = 0 as char;
        } else {
            self.ch = self.next_char();
        }

        self.start_position = self.read_position;
        self.read_position += 1;
    }

    fn next_char(&self) -> char {
        self.input
            .chars()
            .nth(self.read_position)
            .unwrap_or_else(|| {
                panic!(
                    "[Lexer Error]: Token out of range at index `{}`",
                    &self.read_position
                )
            })
    }

    fn peek_char(&self) -> u8 {
        if self.read_position >= self.input.len() {
            return 0;
        } else {
            return self.input.as_bytes()[self.read_position];
        }
    }
}
