use crate::{macros, IntoBoundedStatic, ToBoundedStatic};

macros::make_clone_impl!(ahash::RandomState);

/// Blanket [`ToBoundedStatic`] impl for converting `ahash::AHashMap<K, V, S>` to `ahash::AHashMap<K, V, S>: 'static`.
#[cfg(feature = "std")]
impl<K, V, S> ToBoundedStatic for ahash::AHashMap<K, V, S>
where
    K: ToBoundedStatic,
    K::Static: Eq + std::hash::Hash,
    V: ToBoundedStatic,
    S: ToBoundedStatic,
    S::Static: std::hash::BuildHasher,
{
    type Static = ahash::AHashMap<K::Static, V::Static, S::Static>;

    fn to_static(&self) -> Self::Static {
        let mut map =
            ahash::AHashMap::with_capacity_and_hasher(self.len(), self.hasher().to_static());
        map.extend(self.iter().map(|(k, v)| (k.to_static(), v.to_static())));
        map
    }
}

/// Blanket [`IntoBoundedStatic`] impl for converting `ahash::AHashMap<K, V, S>` into `ahash::AHashMap<K, V, S>: 'static`.
#[cfg(feature = "std")]
impl<K, V, S> IntoBoundedStatic for ahash::AHashMap<K, V, S>
where
    K: IntoBoundedStatic,
    K::Static: Eq + std::hash::Hash,
    V: IntoBoundedStatic,
    S: ToBoundedStatic,
    S::Static: std::hash::BuildHasher,
{
    type Static = ahash::AHashMap<K::Static, V::Static, S::Static>;

    fn into_static(self) -> Self::Static {
        let mut map =
            ahash::AHashMap::with_capacity_and_hasher(self.len(), self.hasher().to_static());
        map.extend(
            self.into_iter()
                .map(|(k, v)| (k.into_static(), v.into_static())),
        );
        map
    }
}

/// Blanket [`ToBoundedStatic`] impl for converting `ahash::AHashSet<T, S>` to `ahash::AHashSet<T, S>: 'static`.
#[cfg(feature = "std")]
impl<T, S> ToBoundedStatic for ahash::AHashSet<T, S>
where
    T: ToBoundedStatic,
    T::Static: Eq + std::hash::Hash,
    S: ToBoundedStatic,
    S::Static: std::hash::BuildHasher,
{
    type Static = ahash::AHashSet<T::Static, S::Static>;

    fn to_static(&self) -> Self::Static {
        let mut set =
            ahash::AHashSet::with_capacity_and_hasher(self.len(), self.hasher().to_static());
        set.extend(self.iter().map(ToBoundedStatic::to_static));
        set
    }
}

/// Blanket [`IntoBoundedStatic`] impl for converting `ahash::AHashSet<T, S>` into `ahash::AHashSet<T, S>: 'static`.
#[cfg(feature = "std")]
impl<T, S> IntoBoundedStatic for ahash::AHashSet<T, S>
where
    T: IntoBoundedStatic,
    T::Static: Eq + std::hash::Hash,
    S: ToBoundedStatic,
    S::Static: std::hash::BuildHasher,
{
    type Static = ahash::AHashSet<T::Static, S::Static>;

    fn into_static(self) -> Self::Static {
        let mut set =
            ahash::AHashSet::with_capacity_and_hasher(self.len(), self.hasher().to_static());
        set.extend(self.into_iter().map(IntoBoundedStatic::into_static));
        set
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "std")]
    use alloc::{borrow::Cow, string::String};

    fn ensure_static<T: 'static>(t: T) {
        drop(t);
    }

    #[test]
    fn test_ahash_random_state() {
        ensure_static(ahash::RandomState::new().to_static());
    }

    #[cfg(feature = "std")]
    #[test]
    fn test_ahash_ahashmap() {
        let k = String::from("key");
        let v = String::from("value");
        let value = ahash::AHashMap::from([(Cow::from(&k), Cow::from(&v))]);
        let to_static = value.to_static();
        ensure_static(to_static);
    }

    #[cfg(feature = "std")]
    #[test]
    fn test_ahash_ahashset() {
        let value = String::from("data");
        let value = ahash::AHashSet::from([(Cow::from(&value))]);
        let to_static = value.to_static();
        ensure_static(to_static);
    }
}
