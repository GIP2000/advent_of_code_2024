#[derive(Clone)]
pub struct StrReader<'a> {
    val: &'a str,
    index: usize,
}

impl<'a> StrReader<'a> {
    pub fn new(val: &'a str) -> Self {
        Self { val, index: 0 }
    }

    pub fn peek(&self) -> Option<char> {
        self.val.chars().nth(self.index)
    }

    pub fn read(&mut self) -> Option<char> {
        let result = self.val.chars().nth(self.index);
        if let Some(_) = result {
            self.index += 1
        }
        return result;
    }

    pub fn consume(&mut self, count: usize) {
        self.index = (self.index + count).min(self.val.len());
    }

    pub fn act_on_slice<Return, Closure>(&self, callback: Closure) -> Return
    where
        Closure: Fn(&str) -> Return,
    {
        return (callback)(&self.val[self.index..]);
    }

    pub fn get_quoted(&mut self) -> Option<&'a str> {
        let mut str_reader = self.clone();
        let starts_with_quote = str_reader.read().map(|x| x == '"').unwrap_or(false);
        let start = str_reader.index;
        if !starts_with_quote {
            return None;
        }
        loop {
            let val = if let Some(val) = str_reader.read() {
                val
            } else {
                return None;
            };
            let peek = str_reader.peek();

            match (val, peek) {
                ('\\', Some('"')) => {}
                (_, Some('"')) => {
                    str_reader.consume(1);
                    *self = str_reader;
                    return Some(&self.val[start..self.index - 1]);
                }
                _ => {}
            };
        }
    }

    pub fn consume_until_end_paren(&mut self) -> bool {
        let mut str_reader = self.clone();
        let mut p_count = 0;
        let mut in_quotes = false;
        loop {
            let val = if let Some(val) = str_reader.read() {
                val
            } else {
                return false;
            };
            let peek = str_reader.peek();

            match (val, peek, in_quotes) {
                ('(', _, false) => p_count += 1,

                (')', _, false) => p_count -= 1,

                ('"', _, false) => in_quotes = true,

                ('\\', Some('"'), true) => str_reader.consume(1),

                (_, Some('"'), true) => {
                    in_quotes = false;
                    str_reader.consume(1);
                }
                _ => {}
            };

            if p_count == -1 {
                *self = str_reader;
                return true;
            }
        }
    }
}
