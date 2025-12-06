#[macro_export]
macro_rules! include_bytes_aligned {
    ($path:literal, $alignment:expr) => {{
        ::aligned_vec::AVec::<_, ::aligned_vec::ConstAlign<{ $alignment }>>::from_slice(
            $alignment,
            include_bytes!($path),
        )
    }};
    ($path:literal) => {
        include_bytes_aligned!($path, $crate::raw::ALIGNMENT)
    };
}

/// Copies the given bytes to an array and fills the leftovers with `b'\0''`
pub fn test_string<const N: usize>(val: &[u8]) -> [i8; N] {
    assert!(N > val.len());
    let mut arr = [0; N];
    arr[0..val.len()].clone_from_slice(val);

    arr.map(|b| b as i8)
}
