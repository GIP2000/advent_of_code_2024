pub struct LazyBuffer<I>
where
    I: Iterator,
{
    iter: I,
    buf: Vec<I::Item>,
}

impl<I> LazyBuffer<I>
where
    I: Iterator,
{
    pub fn new(iter: I) -> Self {
        let (low, high) = iter.size_hint();
        let size = if let Some(high) = high { high } else { low };
        let buf = Vec::with_capacity(size);
        Self { iter, buf }
    }

    pub fn get<'a>(&'a mut self, index: usize) -> Option<&'a I::Item> {
        if index < self.buf.len() {
            return Some(&self.buf[index]);
        }

        let rest = index - self.buf.len();
        for val in self.iter.by_ref().take(rest + 1) {
            self.buf.push(val);
        }
        self.buf.get(index)
    }
}
