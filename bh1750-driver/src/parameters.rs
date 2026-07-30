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

pub(crate) enum Mode {
    OneShot = 0b0010,
    Continuous = 0b0001,
}
