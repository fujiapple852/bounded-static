use crate::{IntoBoundedStatic, ToBoundedStatic};

/// [`ToBoundedStatic`] impl for `smol_str::SmolStr`.
impl ToBoundedStatic for smol_str::SmolStr {
    type Static = Self;

    fn to_static(&self) -> Self::Static {
        self.clone()
    }
}

/// No-op [`IntoBoundedStatic`] impl for `smol_str::SmolStr`.
impl IntoBoundedStatic for smol_str::SmolStr {
    type Static = Self;

    fn into_static(self) -> Self::Static {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ensure_static<T: 'static>(t: T) {
        drop(t);
    }

    #[test]
    fn test_smol_str() {
        ensure_static(smol_str::SmolStr::new("smol").to_static());
        ensure_static(smol_str::SmolStr::new("smol").into_static());
    }
}
