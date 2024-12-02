#[derive(Clone)]
pub struct SlidingWindowIterator<T, I, const WINDOW_SIZE: usize>
where
    T: Clone,
    I: Iterator<Item = T>,
{
    iter: I,
    window: Option<[T; WINDOW_SIZE]>,
}

impl<T, I, const WINDOW_SIZE: usize> SlidingWindowIterator<T, I, WINDOW_SIZE>
where
    T: Clone,
    I: Iterator<Item = T>,
{
    pub fn new(iter: I) -> Self {
        Self { iter, window: None }
    }
}

impl<T, I, const WINDOW_SIZE: usize> Iterator for SlidingWindowIterator<T, I, WINDOW_SIZE>
where
    T: Clone,
    I: Iterator<Item = T>,
{
    type Item = [T; WINDOW_SIZE];

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

pub trait SlidingWindowIteratorTrait<T: Clone>: Iterator<Item = T> + Sized {
    fn sliding_window<const WINDOW_SIZE: usize>(
        self,
    ) -> SlidingWindowIterator<T, Self, WINDOW_SIZE> {
        SlidingWindowIterator::new(self)
    }
}

impl<T: Clone, I: Iterator<Item = T>> SlidingWindowIteratorTrait<T> for I {}

#[cfg(test)]
mod test {

    use super::*;

    #[derive(Debug)]
    struct A(usize);

    #[test]
    fn test() {
        let arr: [A; 20] = std::array::from_fn(|i| A(i));
        println!("arr = {arr:?}");
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

        println!("a = {:?}", a.last().map(|b| b));
    }

    #[test]
    fn test2() {
        let mut arr: [A; 20] = std::array::from_fn(|i| A(i));
        // arr.iter_mut().sliding_window::<3>();
    }
}
