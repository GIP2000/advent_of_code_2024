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
        self.val.as_bytes().get(self.index).map(|f| *f as char)
        // This would be neccisary if the str wasn't ascii but the above is O(1) so...
        // self.val.chars().nth(self.index)
    }

    pub fn read(&mut self) -> Option<char> {
        let result = self.val.as_bytes().get(self.index).map(|f| *f as char);
        // This would be neccisary if the str wasn't ascii but the above is O(1) so...
        // let result = self.val.chars().nth(self.index);
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

    pub fn gen_iter<R, F: Fn(char, &mut Self) -> Option<R>>(
        self,
        gen_fn: F,
    ) -> StrReaderGenerater<'a, R, F> {
        StrReaderGenerater::new(self, gen_fn)
    }
}

pub struct StrReaderGenerater<'a, R, F>
where
    F: Fn(char, &mut StrReader<'a>) -> Option<R>,
{
    str_reader: StrReader<'a>,
    gen_fn: F,
}

impl<'a, R, F> StrReaderGenerater<'a, R, F>
where
    F: Fn(char, &mut StrReader<'a>) -> Option<R>,
{
    pub fn new(str_reader: StrReader<'a>, gen_fn: F) -> Self {
        Self { str_reader, gen_fn }
    }
}

impl<'a, R, F> Iterator for StrReaderGenerater<'a, R, F>
where
    F: Fn(char, &mut StrReader<'a>) -> Option<R>,
{
    type Item = R;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(c) = self.str_reader.read() {
            let result = (self.gen_fn)(c, &mut self.str_reader);
            if result.is_some() {
                return result;
            }
        }
        None
    }
}
