use crate::{IntoBoundedStatic, ToBoundedStatic};

/// Blanket [`ToBoundedStatic`] impl for converting `HashMap<K, V>` to `HashMap<K, V>: 'static`.
impl<K, V, S> ToBoundedStatic for std::collections::HashMap<K, V, S>
where
    K: ToBoundedStatic,
    K::Static: Eq + std::hash::Hash,
    V: ToBoundedStatic,
    S: ToBoundedStatic,
    S::Static: std::hash::BuildHasher,
{
    type Static = std::collections::HashMap<K::Static, V::Static, S::Static>;

    fn to_static(&self) -> Self::Static {
        let mut map = std::collections::HashMap::with_capacity_and_hasher(
            self.len(),
            self.hasher().to_static(),
        );
        map.extend(self.iter().map(|(k, v)| (k.to_static(), v.to_static())));
        map
    }
}

/// Blanket [`IntoBoundedStatic`] impl for for converting `HashMap<K, V>` into `HashMap<K, V>: 'static`.
impl<K, V, S> IntoBoundedStatic for std::collections::HashMap<K, V, S>
where
    K: IntoBoundedStatic,
    K::Static: Eq + std::hash::Hash,
    V: IntoBoundedStatic,
    S: ToBoundedStatic,
    S::Static: std::hash::BuildHasher,
{
    type Static = std::collections::HashMap<K::Static, V::Static, S::Static>;

    fn into_static(self) -> Self::Static {
        let mut map = std::collections::HashMap::with_capacity_and_hasher(
            self.len(),
            self.hasher().to_static(),
        );
        map.extend(
            self.into_iter()
                .map(|(k, v)| (k.into_static(), v.into_static())),
        );
        map
    }
}

/// Blanket [`ToBoundedStatic`] impl for converting `HashSet<T>` into `HashSet<T>: 'static`.
impl<T, S> ToBoundedStatic for std::collections::HashSet<T, S>
where
    T: ToBoundedStatic,
    T::Static: Eq + std::hash::Hash,
    S: ToBoundedStatic,
    S::Static: std::hash::BuildHasher,
{
    type Static = std::collections::HashSet<T::Static, S::Static>;

    fn to_static(&self) -> Self::Static {
        let mut set = std::collections::HashSet::with_capacity_and_hasher(
            self.len(),
            self.hasher().to_static(),
        );
        set.extend(self.iter().map(ToBoundedStatic::to_static));
        set
    }
}

/// Blanket [`IntoBoundedStatic`] impl for converting `HashSet<T>` into `HashSet<T>: 'static`.
impl<T, S> IntoBoundedStatic for std::collections::HashSet<T, S>
where
    T: IntoBoundedStatic,
    T::Static: Eq + std::hash::Hash,
    S: ToBoundedStatic,
    S::Static: std::hash::BuildHasher,
{
    type Static = std::collections::HashSet<T::Static, S::Static>;

    fn into_static(self) -> Self::Static {
        let mut set = std::collections::HashSet::with_capacity_and_hasher(
            self.len(),
            self.hasher().to_static(),
        );
        set.extend(self.into_iter().map(IntoBoundedStatic::into_static));
        set
    }
}

/// [`ToBoundedStatic`] impl for `std::collections::hash_map::RandomState`.
impl ToBoundedStatic for std::collections::hash_map::RandomState {
    type Static = Self;

    fn to_static(&self) -> Self::Static {
        self.clone()
    }
}

#[cfg(test)]
mod tests {
    use core::any::Any;

    use super::*;
    use alloc::{borrow::Cow, string::String};

    fn ensure_static<T: 'static>(t: T) {
        drop(t);
    }

    #[test]
    fn test_hashmap1() {
        let k = String::from("key");
        let v = String::from("value");
        let value = std::collections::HashMap::from([(Cow::from(&k), Cow::from(&v))]);
        let to_static = value.to_static();
        ensure_static(to_static);
    }

    #[test]
    fn test_hashmap2() {
        let k = "key";
        let v = String::from("value");
        let value = std::collections::HashMap::from([(k, Cow::from(&v))]);
        let to_static = value.to_static();
        ensure_static(to_static);
    }

    #[test]
    fn test_hashmap3() {
        let k = String::from("key");
        let v = 0i16;
        let value = std::collections::HashMap::from([(Cow::from(&k), v)]);
        let to_static = value.to_static();
        ensure_static(to_static);
    }

    #[test]
    fn test_hashset() {
        let value = String::from("data");
        let value = std::collections::HashSet::from([(Cow::from(&value))]);
        let to_static = value.to_static();
        ensure_static(to_static);
    }

    #[test]
    fn test_custom_random_state() {
        #[derive(Clone, Default)]
        struct RandomState;

        impl std::hash::BuildHasher for RandomState {
            type Hasher = std::collections::hash_map::DefaultHasher;

            fn build_hasher(&self) -> Self::Hasher {
                std::collections::hash_map::DefaultHasher::default()
            }
        }

        impl ToBoundedStatic for RandomState {
            type Static = Self;

            fn to_static(&self) -> Self::Static {
                self.clone()
            }
        }

        let k = "key";
        let v = 0i16;
        let value = std::collections::HashMap::<_, _, RandomState>::from_iter([(k, v)]);
        let to_static = value.to_static();
        assert_eq!(value.type_id(), to_static.type_id());
        ensure_static(to_static);
        let value = std::collections::HashSet::<_, RandomState>::from_iter([k]);
        let to_static = value.to_static();
        assert_eq!(value.type_id(), to_static.type_id());
        ensure_static(to_static);
    }
}
