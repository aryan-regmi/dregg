/// A trait to represent a range.
pub trait RangeTrait<T> {
    /// The starting point of the range.
    fn start(&self) -> T;

    /// The ending point of the range.
    fn end(&self) -> T;

    /// Creates a new instance with the same value for `start` and `end`.
    fn singular(value: T) -> Self;

    /// Determines if an instance is singular (i.e. starts and ends on the same value).
    fn is_singular(&self) -> bool;
}

// Represents a range of possible values.
#[derive(Debug, Clone, PartialEq)]
pub struct Range<T> {
    pub start: T,
    pub end: T,
}

impl<T> RangeTrait<T> for Range<T>
where
    T: Clone + PartialEq,
{
    fn start(&self) -> T {
        self.start.clone()
    }

    fn end(&self) -> T {
        self.end.clone()
    }

    fn singular(value: T) -> Self {
        Self {
            start: value.clone(),
            end: value,
        }
    }

    fn is_singular(&self) -> bool {
        self.start == self.end
    }
}

/// Represents a range of values under the `start` value and over the `end` value.
#[derive(Debug)]
pub struct UnderOver<T> {
    pub under: T,
    pub over: T,
}

impl<T> RangeTrait<T> for UnderOver<T>
where
    T: Clone + PartialEq,
{
    fn start(&self) -> T {
        self.under.clone()
    }

    fn end(&self) -> T {
        self.over.clone()
    }

    fn singular(value: T) -> Self {
        Self {
            under: value.clone(),
            over: value,
        }
    }

    fn is_singular(&self) -> bool {
        self.under == self.over
    }
}
