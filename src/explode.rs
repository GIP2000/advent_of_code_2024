use std::vec::IntoIter;
pub enum ExplodeStates<I>
where
    I: Iterator,
{
    Setup(I),
    Iterate(IntoIter<I::Item>),
}

pub struct ExplodeOps<I, F, const O: usize>
where
    I: Iterator,
    F: Fn(&I::Item, &I::Item) -> [I::Item; O],
{
    inner: ExplodeStates<I>,
    closure: F,
}

impl<I, F, const O: usize> Iterator for ExplodeOps<I, F, O>
where
    I: Iterator,
    F: Fn(&I::Item, &I::Item) -> [I::Item; O],
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.inner {
            ExplodeStates::Setup(iter) => {
                let first_val = iter.next()?;

                let prev = iter.fold(vec![first_val], |prev, val| {
                    prev.iter()
                        .flat_map(|prev_val| (self.closure)(prev_val, &val))
                        .collect()
                });
                let mut new_iter = prev.into_iter();
                let val = new_iter.next();
                self.inner = ExplodeStates::Iterate(new_iter);
                val
            }
            ExplodeStates::Iterate(prev_iter) => prev_iter.next(),
        }
    }
}

pub trait ExplodeIter<I, F, const O: usize>
where
    I: Iterator,
    F: Fn(&I::Item, &I::Item) -> [I::Item; O],
{
    fn explode(self, closure: F) -> ExplodeOps<I, F, O>;
}

impl<I, F, const O: usize> ExplodeIter<I, F, O> for I
where
    I: Iterator,
    F: Fn(&I::Item, &I::Item) -> [I::Item; O],
{
    /// This function consumes and uses the entire iterator at once
    /// and explodes out the value
    /// Example:
    /// ```rust
    /// use advent_of_code_2024::explode::ExplodeIter;
    /// let result = [1, 2, 3, 4]
    ///     .into_iter()
    ///     .explode(|prev, val| [prev + val, prev * val])
    ///     .collect::<Vec<_>>();
    ///
    /// println!("{:?}", result); // This would be [10, 24, 13, 36, 9, 20, 10, 24]
    /// ```
    /// Explanation:
    /// \[1]
    /// \[1 + 2, 1 * 2]                                       -> \[3, 2]
    /// \[3 + 3,3 * 3, 2 + 3,2 * 3]                           -> \[6, 9, 5, 6]
    /// \[6 + 4,6 * 5, 9 + 5,9 * 4, 5 + 4,5 * 4, 6 + 4,6 * 4] -> \[10, 24, 13, 36, 9, 20, 10, 24]
    fn explode(self, closure: F) -> ExplodeOps<I, F, O> {
        ExplodeOps {
            inner: ExplodeStates::Setup(self),
            closure,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let result = [1, 2, 3, 4]
            .into_iter()
            .explode(|prev, val| [prev + val, prev * val])
            .collect::<Vec<_>>();

        assert_eq!(result, vec![10, 24, 13, 36, 9, 20, 10, 24]);
    }
}
