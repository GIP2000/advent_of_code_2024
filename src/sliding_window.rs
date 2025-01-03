#[derive(Clone)]
pub struct SlidingWindowIterator<I, const WINDOW_SIZE: usize>
where
    I: Iterator,
{
    iter: I,
    window: Option<[I::Item; WINDOW_SIZE]>,
}

impl<I, const WINDOW_SIZE: usize> SlidingWindowIterator<I, WINDOW_SIZE>
where
    I: Iterator,
    I::Item: Clone,
{
    pub fn new(iter: I) -> Self {
        Self { iter, window: None }
    }
}

impl<I, const WINDOW_SIZE: usize> Iterator for SlidingWindowIterator<I, WINDOW_SIZE>
where
    I: Iterator,
    I::Item: Clone,
{
    type Item = [I::Item; WINDOW_SIZE];

    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.window {
            Some(prev_window) => {
                prev_window.rotate_right(1);
                prev_window[0] = self.iter.next()?;
            }
            None => {
                let mut arr = self.iter.next_chunk().ok()?;
                arr.reverse();
                self.window = Some(arr);
            }
        };

        self.window.clone()
    }
}

pub trait SlidingWindowIteratorTrait: Iterator + Sized
where
    Self::Item: Clone,
{
    fn sliding_window<const WINDOW_SIZE: usize>(self) -> SlidingWindowIterator<Self, WINDOW_SIZE> {
        SlidingWindowIterator::new(self)
    }
}

impl<I> SlidingWindowIteratorTrait for I
where
    I: Iterator,
    I::Item: Clone,
{
}

#[cfg(test)]
mod test {

    use super::*;

    #[derive(Debug, PartialEq, Eq)]
    struct A(usize);
    #[test]
    fn test_mut() {
        let arr: [A; 20] = std::array::from_fn(|i| A(i));
    }

    #[test]
    fn test() {
        let arr: [A; 20] = std::array::from_fn(|i| A(i));

        let a = arr
            .iter()
            .sliding_window()
            .map(|[a, b, c]| {
                println!("a,b,c = {:?},{:?},{:?}", a, b, c);
                return b;
            })
            .map(|b| {
                println!("b = {:?}", b);
                return b.0;
            })
            .collect::<Vec<_>>();

        assert_eq!(
            a,
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18]
        )
    }
}
