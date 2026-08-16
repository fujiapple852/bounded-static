use crate::{macros, IntoBoundedStatic, ToBoundedStatic};

macros::make_clone_impl!(smol_str::SmolStr);

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
