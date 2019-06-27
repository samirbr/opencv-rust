use libc::size_t;

pub trait Vector<'i> {
    type Storage;
    type Arg;

    /// Create a new Vector
    fn new() -> Self where Self: Sized;

    /// Create a Vector from iterator
    #[inline]
    fn from_iter(s: impl IntoIterator<Item=Self::Arg>) -> Self where Self: Sized {
        let s = s.into_iter();
        let (lo, hi) = s.size_hint();
        let mut out = Self::with_capacity(hi.unwrap_or(lo));
        s.for_each(|x| out.push(x));
        out
    }

    /// Create a Vector with pre-defined capacity
    #[inline]
    fn with_capacity(capacity: size_t) -> Self where Self: Sized {
        let mut out = Self::new();
        out.reserve(capacity);
        out
    }

    /// Return Vector length
    fn len(&self) -> size_t;

    /// Return true if Vector is empty
    fn is_empty(&self) -> bool;

    /// Return Vector current capacity
    fn capacity(&self) -> size_t;

    /// Free extra capacity
    fn shrink_to_fit(&mut self);

    /// Reserve capacity for `additinal` new elements
    fn reserve(&mut self, additional: size_t);

    /// Remove all elements
    fn clear(&mut self);

    /// Add new element
    fn push(&mut self, val: Self::Arg);

    /// Insert a new element at the specified `index`
    fn insert(&mut self, index: size_t, val: Self::Arg) -> crate::Result<()>;

    /// Remove the element at the specified `index`
    fn remove(&mut self, index: size_t) -> crate::Result<()>;

    /// Swaps 2 elements in the Vector
    fn swap(&mut self, index1: size_t, index2: size_t) -> crate::Result<()>;

    /// Get element at the specified `index`
    fn get(&self, index: size_t) -> crate::Result<Self::Storage>;

    /// Same as `get()` but without bounds checking
    unsafe fn get_unchecked(&self, index: size_t) -> Self::Storage;

    /// Set element at the specified `index`
    fn set(&mut self, index: size_t, val: Self::Arg) -> crate::Result<()>;

    /// Same as `set()` but without bounds checking
    unsafe fn set_unchecked(&mut self, index: size_t, val: Self::Arg);

    /// Convert to Rust `Vec`
    #[inline]
    fn to_vec(&self) -> Vec<Self::Storage> {
        (0..self.len()).map(|x| unsafe { self.get_unchecked(x) }).collect()
    }
}

impl<S, A> dyn Vector<'_, Storage=S, Arg=A> + '_ {
    #[inline(always)]
    pub(crate) fn index_check(index: size_t, len: size_t) -> crate::Result<()> {
        if index >= len {
            Err(crate::Error::new(crate::core::StsOutOfRange, format!("Index: {} out of bounds: 0..{}", index, len)))
        } else {
            Ok(())
        }
    }
}

pub struct VectorIterator<T> {
    vec: T,
    i: size_t,
}

impl<T> VectorIterator<T> {
    pub fn new(vec: T) -> Self {
        Self { vec, i: 0 }
    }
}

impl<T, S> Iterator for VectorIterator<T>
    where
        T: for<'i> Vector<'i, Storage=S>
{
    type Item = S;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.vec.get(self.i);
        self.i += 1;
        out.ok()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.vec.len(), None)
    }
}

impl<T, S> ExactSizeIterator for VectorIterator<T>
    where
        T: for<'i> Vector<'i, Storage=S>
{
    fn len(&self) -> usize {
        self.vec.len()
    }
}

pub struct VectorRefIterator<'v, T> {
    vec: &'v T,
    i: size_t,
}

impl<'v, T> VectorRefIterator<'v, T> {
    pub fn new(vec: &'v T) -> Self {
        Self { vec, i: 0 }
    }
}

impl<T, S> Iterator for VectorRefIterator<'_, T>
    where
        T: for<'i> Vector<'i, Storage=S>,
{
    type Item = S;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.vec.get(self.i);
        self.i += 1;
        out.ok()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.vec.len(), None)
    }
}

impl<T, S> ExactSizeIterator for VectorRefIterator<'_, T>
    where
        T: for<'i> Vector<'i, Storage=S>
{
    fn len(&self) -> usize {
        self.vec.len()
    }
}