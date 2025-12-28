#[derive(Clone, Copy, Debug)]
pub enum Bitfield {
    EngineWarnings(EngineWarnings),
    Unknown(u32),
}

impl Bitfield {
    pub(crate) fn parse(val: u32, unit: &str) -> Self {
        match unit {
            "irsdk_EngineWarnings" => Self::engine_warnings(val),
            _ => Self::Unknown(val),
        }
    }

    fn engine_warnings(raw: u32) -> Self {
        Self::EngineWarnings(EngineWarnings(raw))
    }
}

/// Defines a tuple struct holding a `u32`, where each bit in the value may represent a different
/// "flag". An accessor for each flag is generated, which masks the value to extract the flag's
/// specified bit, returning `true` if the bit is set. A `Debug` impl is also generated.
macro_rules! bitfield {
    ($name:ident { $($bit:literal => $field:ident),+ $(,)? }) => {
        #[derive(Clone, Copy)]
        pub struct $name(u32);

        impl $name {
            $(
                pub fn $field(&self) -> bool {
                    self.0 & (1 <<  $bit) > 0
                }
            )+
        }

        impl ::core::fmt::Debug for $name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                let entries = ::bit_iter::BitIter::from(self.0).map(|set_bit| match set_bit {
                    $($bit => stringify!($field),)+
                    _ => "unknown",
                });
                f.debug_set().entries(entries).finish()
            }
        }
    };
}

bitfield! {
    EngineWarnings {
        1 => water_temp,
        2 => fuel_pressure,
        3 => oil_pressure,
        4 => engine_stalled,
        5 => pit_speed_limiter,
        6 => rev_limiter,
        7 => oil_temp,
        8 => mandatory_repairs,
        9 => optional_repairs,
    }
}
