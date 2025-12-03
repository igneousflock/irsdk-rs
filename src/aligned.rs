//! Helper for ensuring raw bytes are aligned

use std::mem::size_of;

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

impl From<Aligned<{ size_of::<u32>() }>> for u32 {
    fn from(wrapper: Aligned<{ size_of::<u32>() }>) -> Self {
        *bytemuck::from_bytes(&wrapper.0)
    }
}

impl From<Aligned<{ size_of::<f32>() }>> for f32 {
    fn from(wrapper: Aligned<{ size_of::<f32>() }>) -> Self {
        *bytemuck::from_bytes(&wrapper.0)
    }
}

impl From<Aligned<{ size_of::<f64>() }>> for f64 {
    fn from(wrapper: Aligned<{ size_of::<f64>() }>) -> Self {
        *bytemuck::from_bytes(&wrapper.0)
    }
}

/// Copy the raw data into an aligned array and reinterpret it as the given type
pub fn align_cast<T, const SIZE: usize>(data: &[u8]) -> T
where
    T: From<Aligned<SIZE>> + bytemuck::Pod,
{
    Aligned::new(data).into()
}
