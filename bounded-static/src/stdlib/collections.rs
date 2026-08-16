use crate::{IntoBoundedStatic, ToBoundedStatic};
use alloc::collections::{BTreeMap, BTreeSet, BinaryHeap, LinkedList, VecDeque};

/// Blanket [`ToBoundedStatic`] impl for converting `BinaryHeap<T>` into `BinaryHeap<T>: 'static`.
impl<T> ToBoundedStatic for BinaryHeap<T>
where
    T: ToBoundedStatic,
    T::Static: Ord,
{
    type Static = BinaryHeap<T::Static>;

    fn to_static(&self) -> Self::Static {
        self.iter().map(ToBoundedStatic::to_static).collect()
    }
}

/// Blanket [`IntoBoundedStatic`] impl for converting `BinaryHeap<T>` into `BinaryHeap<T>: 'static`.
impl<T> IntoBoundedStatic for BinaryHeap<T>
where
    T: IntoBoundedStatic,
    T::Static: Ord,
{
    type Static = BinaryHeap<T::Static>;

    fn into_static(self) -> Self::Static {
        self.into_iter()
            .map(IntoBoundedStatic::into_static)
            .collect()
    }
}

/// Blanket [`ToBoundedStatic`] impl for converting `BTreeMap<K, V>` into `BTreeMap<K, V>: 'static`.
impl<K, V> ToBoundedStatic for BTreeMap<K, V>
where
    K: ToBoundedStatic,
    K::Static: Ord,
    V: ToBoundedStatic,
{
    type Static = BTreeMap<K::Static, V::Static>;

    fn to_static(&self) -> Self::Static {
        self.iter()
            .map(|(k, v)| (k.to_static(), v.to_static()))
            .collect()
    }
}

/// Blanket [`IntoBoundedStatic`] impl for converting `BTreeMap<K, V>` into `BTreeMap<K, V>: 'static`.
impl<K, V> IntoBoundedStatic for BTreeMap<K, V>
where
    K: IntoBoundedStatic,
    K::Static: Ord,
    V: IntoBoundedStatic,
{
    type Static = BTreeMap<K::Static, V::Static>;

    fn into_static(self) -> Self::Static {
        self.into_iter()
            .map(|(k, v)| (k.into_static(), v.into_static()))
            .collect()
    }
}

/// Blanket [`ToBoundedStatic`] impl for converting `BTreeSet<T>` into `BTreeSet<T>: 'static`.
impl<T> ToBoundedStatic for BTreeSet<T>
where
    T: ToBoundedStatic,
    T::Static: Ord,
{
    type Static = BTreeSet<T::Static>;

    fn to_static(&self) -> Self::Static {
        self.iter().map(ToBoundedStatic::to_static).collect()
    }
}

/// Blanket [`IntoBoundedStatic`] impl for converting `BTreeSet<T>` into `BTreeSet<T>: 'static`.
impl<T> IntoBoundedStatic for BTreeSet<T>
where
    T: IntoBoundedStatic,
    T::Static: Ord,
{
    type Static = BTreeSet<T::Static>;

    fn into_static(self) -> Self::Static {
        self.into_iter()
            .map(IntoBoundedStatic::into_static)
            .collect()
    }
}

/// Blanket [`ToBoundedStatic`] impl for converting `LinkedList<T>` into `LinkedList<T>: 'static`.
impl<T> ToBoundedStatic for LinkedList<T>
where
    T: ToBoundedStatic,
{
    type Static = LinkedList<T::Static>;

    fn to_static(&self) -> Self::Static {
        self.iter().map(ToBoundedStatic::to_static).collect()
    }
}

/// Blanket [`IntoBoundedStatic`] impl for converting `LinkedList<T>` into `LinkedList<T>: 'static`.
impl<T> IntoBoundedStatic for LinkedList<T>
where
    T: IntoBoundedStatic,
{
    type Static = LinkedList<T::Static>;

    fn into_static(self) -> Self::Static {
        self.into_iter()
            .map(IntoBoundedStatic::into_static)
            .collect()
    }
}

/// Blanket [`ToBoundedStatic`] impl for converting `VecDeque<T>` into `VecDeque<T>: 'static`.
impl<T> ToBoundedStatic for VecDeque<T>
where
    T: ToBoundedStatic,
{
    type Static = VecDeque<T::Static>;

    fn to_static(&self) -> Self::Static {
        self.iter().map(ToBoundedStatic::to_static).collect()
    }
}

/// Blanket [`IntoBoundedStatic`] impl for converting `VecDeque<T>` into `VecDeque<T>: 'static`.
impl<T> IntoBoundedStatic for VecDeque<T>
where
    T: IntoBoundedStatic,
{
    type Static = VecDeque<T::Static>;

    fn into_static(self) -> Self::Static {
        self.into_iter()
            .map(IntoBoundedStatic::into_static)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::{borrow::Cow, string::String};

    fn ensure_static<T: 'static>(t: T) {
        drop(t);
    }

    #[test]
    fn test_binary_heap() {
        let s = String::new();
        let value = BinaryHeap::from([Cow::from(&s)]);
        let to_static = value.to_static();
        ensure_static(to_static);
    }

    #[test]
    fn test_btree_map() {
        let k = String::from("key");
        let v = String::from("value");
        let value = BTreeMap::from([(Cow::from(&k), Cow::from(&v))]);
        let to_static = value.to_static();
        ensure_static(to_static);
    }

    #[test]
    fn test_btree_set() {
        let s = String::new();
        let value = BTreeSet::from([Cow::from(&s)]);
        let to_static = value.to_static();
        ensure_static(to_static);
    }

    #[test]
    fn test_linked_list() {
        let s = String::new();
        let value = LinkedList::from([Cow::from(&s)]);
        let to_static = value.to_static();
        ensure_static(to_static);
    }

    #[test]
    fn test_vec_deque() {
        let s = String::new();
        let value = VecDeque::from([Cow::from(&s)]);
        let to_static = value.to_static();
        ensure_static(to_static);
    }
}
