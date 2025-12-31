//! Bitfield types

/// Multiple related telemetry flags compressed into one value
///
/// Internally, these values are 32 bit integers, where each binary bit may represent the state of
/// a certain flag. See each variant's internal type for the possible values.
#[derive(Clone, Copy, Debug)]
pub enum Bitfield {
    EngineWarnings(EngineWarnings),
    Flags(Flags),
    CameraState(CameraState),
    PitServiceFlags(PitServiceFlags),
    PaceFlags(PaceFlags),
    /// The variable's type was `Bitfield` but this crate didn't know how to decode it. Please file
    /// a bug report.
    Unknown(u32),
}

impl Bitfield {
    pub(crate) fn parse_unit(val: u32, unit: &str) -> Self {
        match unit {
            "irsdk_EngineWarnings" => Self::engine_warnings(val),
            "irsdk_Flags" => Self::flags(val),
            "irsdk_CameraState" => Self::camera_state(val),
            "irsdk_PitSvFlags" => Self::pit_service_flags(val),
            "irsdk_PaceFlags" => Self::pace_flags(val),
            _ => Self::Unknown(val),
        }
    }

    fn engine_warnings(raw: u32) -> Self {
        Self::EngineWarnings(EngineWarnings(raw))
    }

    fn flags(raw: u32) -> Self {
        Self::Flags(Flags(raw))
    }

    fn camera_state(raw: u32) -> Self {
        Self::CameraState(CameraState(raw))
    }

    fn pit_service_flags(raw: u32) -> Self {
        Self::PitServiceFlags(PitServiceFlags(raw))
    }

    fn pace_flags(raw: u32) -> Self {
        Self::PaceFlags(PaceFlags(raw))
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
        0 => water_temp,
        1 => fuel_pressure,
        2 => oil_pressure,
        3 => engine_stalled,
        4 => pit_speed_limiter,
        5 => rev_limiter,
        6 => oil_temp,
        7 => mandatory_repairs,
        8 => optional_repairs,
    }
}

bitfield! {
    Flags {
        0 => checkered,
        1 => white,
        2 => green,
        3 => yellow,
        4 => red,
        5 => blue,
        6 => debris,
        7 => crossed,
        8 => yellow_waving,
        9 => one_lap_to_green,
        10 => green_held,
        11 => ten_to_go,
        12 => five_to_go,
        13 => random_waving,
        14 => caution,
        15 => caution_waving,

        16 => black,
        17 => disqualify,
        18 => servicible,
        19 => furled,
        20 => repair,
        21 => disqualified,

        28 => start_lights_hidden,
        29 => start_lights_ready,
        30 => start_lights_set,
        31 => start_lights_go,
    }
}

bitfield! {
    CameraState {
        0 => is_session_screen,
        1 => is_scenic_active,
        3 => camera_tool_active,
        4 => ui_hidden,
        5 => use_auto_shot_selection,
        6 => use_temporary_edits,
        7 => use_key_acceleration,
        8 => use_key_10x_acceleration,
        9 => use_mouse_aim_mode,
    }
}

bitfield! {
    PitServiceFlags {
        0 => lf_tire_change,
        1 => rf_tire_change,
        2 => lr_tire_change,
        3 => rr_tire_change,
        4 => fill_fuel,
        5 => take_winshield_tearoff,
        6 => fast_repair,
    }
}

bitfield! {
    PaceFlags {
        0 => end_of_line,
        1 => free_pass,
        2 => waved_around,
    }
}
