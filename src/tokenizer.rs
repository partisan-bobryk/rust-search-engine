#[derive(Debug)]
pub struct Lexer<'a> {
    content: &'a [char],
}

impl<'a> Lexer<'a> {
    pub fn new(content: &'a [char]) -> Self {
        Self { content }
    }

    fn trim_left(&mut self) {
        // trim whitespace from the left
        while self.content.len() > 0 && self.content[0].is_whitespace() {
            self.content = &self.content[1..];
        }
    }

    fn take(&mut self, n: usize) -> &'a [char] {
        let token = &self.content[0..n];
        self.content = &self.content[n..];
        token
    }

    fn take_while<P>(&mut self, predicate: P) -> &'a [char]
    where
        P: Fn(&char) -> bool,
    {
        let mut n = 0;
        while n < self.content.len() && predicate(&self.content[n]) {
            n += 1;
        }

        self.take(n)
    }

    fn next_token(&mut self) -> Option<&'a [char]> {
        self.trim_left();

        if self.content.is_empty() {
            return None;
        }

        if self.content[0].is_numeric() {
            return Some(self.take_while(|c| c.is_numeric()));
        }

        if self.content[0].is_alphabetic() {
            return Some(self.take_while(|c| c.is_alphanumeric()));
        }

        return Some(self.take(1));
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = &'a [char];

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}
