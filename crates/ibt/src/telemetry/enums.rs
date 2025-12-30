use num_enum::FromPrimitive;

use crate::aligned::align_cast;

#[derive(Clone, Copy, Debug)]
pub enum Enum {
    TrackLocation(TrackLocation),
    TrackSurface(TrackSurface),
    SessionState(SessionState),
    CarLeftRight(CarLeftRight),
    PitServiceStatus(PitServiceStatus),
    PaceMode(PaceMode),
    TrackWetness(TrackWetness),
}

impl Enum {
    pub(crate) fn parse(slice: &[u8], unit: &str) -> Option<Self> {
        let e = match unit {
            "irsdk_TrkLoc" => Self::track_location(align_cast(slice)),
            "irsdk_TrkSurf" => Self::track_surface(align_cast(slice)),
            "irsdk_SessionState" => Self::session_state(align_cast(slice)),
            "irsdk_CarLeftRight" => Self::car_left_right(align_cast(slice)),
            "irsdk_PitSvStatus" => Self::pit_service_status(align_cast(slice)),
            "irsdk_PaceMode" => Self::pace_mode(align_cast(slice)),
            "irsdk_TrackWetness" => Self::track_wetness(align_cast(slice)),

            _ => return None,
        };
        Some(e)
    }

    fn track_location(raw: i32) -> Self {
        Self::TrackLocation(TrackLocation::from(raw))
    }

    fn track_surface(raw: i32) -> Self {
        Self::TrackSurface(TrackSurface::from(raw))
    }

    fn session_state(raw: i32) -> Self {
        Self::SessionState(SessionState::from(raw))
    }

    fn car_left_right(raw: i32) -> Self {
        Self::CarLeftRight(CarLeftRight::from(raw))
    }

    fn pit_service_status(raw: i32) -> Self {
        Self::PitServiceStatus(PitServiceStatus::from(raw))
    }

    fn pace_mode(raw: i32) -> Self {
        Self::PaceMode(PaceMode::from(raw))
    }

    fn track_wetness(raw: i32) -> Self {
        Self::TrackWetness(TrackWetness::from(raw))
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, FromPrimitive)]
#[repr(i32)]
pub enum TrackLocation {
    NotInWorld = -1,
    OffTrack = 0,
    InPitStall,
    /// Includes the lead in to pit road as well as pit road itself. If you just want to know that
    /// you're on the pit road surface, look at the live value `OnPitRoad`.
    ApproachingPits,
    OnTrack,

    #[default]
    Unknown,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, FromPrimitive)]
#[repr(i32)]
pub enum TrackSurface {
    SurfaceNotInWorld = -1,
    #[default]
    Undefined = 0,

    Asphalt1,
    Asphalt2,
    Asphalt3,
    Asphalt4,
    Concrete1,
    Concrete2,
    RacingDirt1,
    RacingDirt2,
    Paint1,
    Paint2,
    Rumble1,
    Rumble2,
    Rumble3,
    Rumble4,

    Grass1,
    Grass2,
    Grass3,
    Grass4,
    Dirt1,
    Dirt2,
    Dirt3,
    Dirt4,
    Sand,
    Gravel1,
    Gravel2,
    Grasscrete,
    Astroturf,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, FromPrimitive)]
#[repr(i32)]
pub enum SessionState {
    #[default]
    Invalid = 0,
    GetInCar,
    Warmup,
    ParadeLaps,
    Racing,
    Checkered,
    Cooldown,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, FromPrimitive)]
#[repr(i32)]
pub enum CarLeftRight {
    #[default]
    Off = 0,
    Clear,
    CarLeft,
    CarRight,
    Middle,
    TwoLeft,
    TwoRight,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, FromPrimitive)]
#[repr(i32)]
pub enum PitServiceStatus {
    #[default]
    None = 0,
    InProgress,
    Complete,
    // errors
    TooFarLeft = 100,
    TooFarRight,
    TooFarForward,
    TooFarBack,
    BadAngle,
    TerminalDamage,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, FromPrimitive)]
#[repr(i32)]
pub enum PaceMode {
    SingleFileStart = 0,
    DoubleFileStart,
    SingleFileRestart,
    DoubleFileRestart,
    #[default]
    NotPacing,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, FromPrimitive)]
#[repr(i32)]
pub enum TrackWetness {
    #[default]
    Unknown,
    Dry,
    MostlyDry,
    VeryLightlyWet,
    LightlyWet,
    ModeratelyWet,
    VeryWet,
    ExtremelyWet,
}
