#[derive(Clone, Copy)]
pub(crate) struct ResolutionSpec {
    pub bits: u8,
    pub delay_ms: u32,
    pub lux_scale: f32,
}

/// Represents the resolution of the sensor
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug)]
pub enum Resolution {
    /// 1 lx, ~120 ms
    High,
    /// 0.5 lx, ~120 ms
    High2,
    /// 4 lx, ~16 ms
    Low,
}

impl Resolution {
    pub(crate) const fn spec(self) -> ResolutionSpec {
        match self {
            Resolution::High => ResolutionSpec {
                bits: 0b0000,
                delay_ms: 180,
                lux_scale: 1.0,
            },
            Resolution::High2 => ResolutionSpec {
                bits: 0b0001,
                delay_ms: 180,
                lux_scale: 0.5,
            },
            Resolution::Low => ResolutionSpec {
                bits: 0b0011,
                delay_ms: 24,
                lux_scale: 1.0,
            },
        }
    }
}

pub(crate) enum Mode {
    OneShot = 0b0010,
    Continuous = 0b0001,
}
