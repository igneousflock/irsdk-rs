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
