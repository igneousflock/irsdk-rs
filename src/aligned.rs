//! Helpers for ensuring raw bytes are aligned

/// Wrapper for raw bytes that forces them to be aligned to `8`, the max alignment for SDK data
/// types
#[repr(align(8))]
pub struct Aligned<const SIZE: usize>([u8; SIZE]);

impl<const SIZE: usize> Aligned<SIZE> {
    /// Copy the data from the slice into a static array with alignment 8
    fn new(slice: &[u8]) -> Self {
        Self(slice.try_into().expect("invalid slice length"))
    }
}

/// Copy the raw data into an aligned array and reinterpret it as the given type
pub fn align_cast<T, const SIZE: usize>(data: &[u8]) -> T
where
    T: From<Aligned<SIZE>> + bytemuck::Pod,
{
    Aligned::new(data).into()
}

macro_rules! impl_from_aligned {
    ($aligned:ident for [$($t:ty),+]) => {
        $(
            impl ::std::convert::From<$aligned<{ ::std::mem::size_of::<$t>() }>> for $t {
                fn from(wrapper: $aligned<{ ::std::mem::size_of::<$t>() }>) -> Self {
                    *(::bytemuck::from_bytes(&wrapper.0))
                }
            }
        )+
    };
}

impl_from_aligned! { Aligned for [u32, f32, f64] }
