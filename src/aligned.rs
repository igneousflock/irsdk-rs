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

impl From<Aligned<4>> for u32 {
    fn from(wrapper: Aligned<4>) -> Self {
        *bytemuck::from_bytes(&wrapper.0)
    }
}

impl From<Aligned<4>> for f32 {
    fn from(wrapper: Aligned<4>) -> Self {
        *bytemuck::from_bytes(&wrapper.0)
    }
}

impl From<Aligned<8>> for f64 {
    fn from(wrapper: Aligned<8>) -> Self {
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
