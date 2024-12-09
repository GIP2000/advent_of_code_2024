use std::{
    convert::Infallible,
    ops::{ControlFlow, FromResidual, Try},
};

pub enum OptionIter<I: Iterator> {
    Some(I),
    None,
}

impl<I: Iterator> OptionIter<I> {
    pub fn as_mut(&mut self) -> OptionIter<&mut I> {
        use OptionIter::*;
        match self {
            Some(x) => Some(x.by_ref()),
            None => None,
        }
    }
}

impl<I: Iterator> Iterator for OptionIter<I> {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.as_mut()?.next()
    }
}

// This lets me do OptionIter<I>? when the return type is Option<T>
// This seems sketchy and wrong
impl<T, I: Iterator> FromResidual<OptionIter<I>> for Option<T> {
    fn from_residual(residual: OptionIter<I>) -> Self {
        match residual {
            OptionIter::Some(_) => panic!("This is infallible"),
            OptionIter::None => None,
        }
    }
}

// This lets me do a ? on an Opiton<T> when I return an OptionIter<I>
impl<I: Iterator> FromResidual<Option<Infallible>> for OptionIter<I> {
    fn from_residual(residual: Option<Infallible>) -> Self {
        match residual {
            Some(_) => panic!("This is infallible"),
            None => Self::None,
        }
    }
}

// this lets me do ? on an OptionIter<I> when I return OptionIter<I>
impl<I: Iterator> FromResidual for OptionIter<I> {
    fn from_residual(residual: <Self as Try>::Residual) -> Self {
        residual
    }
}

impl<I: Iterator> Try for OptionIter<I> {
    type Output = I;

    type Residual = Self;

    fn from_output(output: Self::Output) -> Self {
        Self::Some(output)
    }

    fn branch(self) -> ControlFlow<Self::Residual, Self::Output> {
        match self {
            OptionIter::Some(i) => ControlFlow::Continue(i),
            OptionIter::None => ControlFlow::Break(OptionIter::None),
        }
    }
}

impl<I: Iterator> From<Option<I>> for OptionIter<I> {
    fn from(value: Option<I>) -> Self {
        match value {
            Some(i) => Self::Some(i),
            None => Self::None,
        }
    }
}

impl<I: Iterator> From<OptionIter<I>> for Option<I> {
    fn from(value: OptionIter<I>) -> Self {
        match value {
            OptionIter::Some(i) => Some(i),
            OptionIter::None => None,
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test() {
        let a = test_question_mark_func();
        assert!(Option::<std::vec::IntoIter<usize>>::from(a).is_none());
    }

    fn test_question_mark_func() -> OptionIter<std::vec::IntoIter<usize>> {
        let b = None;
        let _ = b?;
        OptionIter::Some(vec![1, 2, 3].into_iter())
    }
}
