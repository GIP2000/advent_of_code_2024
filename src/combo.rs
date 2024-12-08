use crate::lazy_buffer::LazyBuffer;

pub struct ComboIterator<I>
where
    I: Iterator + Clone,
    I::Item: Clone,
{
    outer_index: usize,
    inner_index: usize,
    lazy_buf: LazyBuffer<I>,
}

impl<I> Iterator for ComboIterator<I>
where
    I: Iterator + Clone,
    I::Item: Clone,
{
    type Item = (I::Item, I::Item);

    fn next(&mut self) -> Option<Self::Item> {
        let inner = if let Some(inner) = self.lazy_buf.get(self.inner_index) {
            inner.clone()
        } else {
            self.outer_index += 1;
            self.inner_index = self.outer_index + 1;
            self.lazy_buf.get(self.inner_index)?.clone()
        };

        let outer = self.lazy_buf.get(self.outer_index)?.clone();
        self.inner_index += 1;

        Some((outer, inner))
    }
}

pub trait ComboIterTrait<I>
where
    I: Iterator + Clone,
    I::Item: Clone,
{
    fn combo(self) -> ComboIterator<I>;
}
impl<I> ComboIterTrait<I> for I
where
    I: Iterator + Clone,
    I::Item: Clone,
{
    fn combo(self) -> ComboIterator<I> {
        ComboIterator {
            lazy_buf: LazyBuffer::new(self),
            outer_index: 0,
            inner_index: 1,
        }
    }
}
