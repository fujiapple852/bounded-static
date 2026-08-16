use crate::{IntoBoundedStatic, ToBoundedStatic};

/// [`ToBoundedStatic`] impl for `smallvec::SmallVec`.
impl<A, T> ToBoundedStatic for smallvec::SmallVec<A>
where
    A: smallvec::Array<Item = T> + ToBoundedStatic,
    T: ToBoundedStatic,
    <A as ToBoundedStatic>::Static: smallvec::Array<Item = T::Static>,
{
    type Static = smallvec::SmallVec<A::Static>;

    fn to_static(&self) -> Self::Static {
        self.iter().map(ToBoundedStatic::to_static).collect()
    }
}

/// [`IntoBoundedStatic`] impl for `smallvec::SmallVec`.
impl<A, T> IntoBoundedStatic for smallvec::SmallVec<A>
where
    A: smallvec::Array<Item = T> + IntoBoundedStatic,
    T: IntoBoundedStatic,
    <A as IntoBoundedStatic>::Static: smallvec::Array<Item = T::Static>,
{
    type Static = smallvec::SmallVec<A::Static>;

    fn into_static(self) -> Self::Static {
        self.into_iter()
            .map(IntoBoundedStatic::into_static)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "alloc")]
    use alloc::{borrow::Cow, string::String};

    fn ensure_static<T: 'static>(t: T) {
        drop(t);
    }

    #[test]
    fn test_smallvec1() {
        let vec: smallvec::SmallVec<[usize; 0]> = smallvec::SmallVec::new();
        ensure_static(vec.to_static());
        ensure_static(vec.into_static());
    }

    #[test]
    fn test_smallvec2() {
        let buf = [1, 2, 3, 4, 5];
        let small_vec: smallvec::SmallVec<_> = smallvec::SmallVec::from_buf(buf);
        ensure_static(small_vec.to_static());
        ensure_static(small_vec.into_static());
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_smallvec3() {
        let x = String::from("foo");
        let y = String::from("bar");
        let buf = [Cow::Borrowed(x.as_str()), Cow::Borrowed(y.as_str())];
        let small_vec: smallvec::SmallVec<_> = smallvec::SmallVec::from_buf(buf);
        ensure_static(small_vec.to_static());
        ensure_static(small_vec.into_static());
    }
}
