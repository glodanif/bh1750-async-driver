use crate::parameters::ResolutionSpec;
use crate::Resolution;

pub const DEFAULT_MT_REG: u8 = 69;

pub struct Config {
    resolution: Resolution,
    pub(crate) mt_reg: u8,
}

impl Config {
    pub fn new(resolution: Resolution) -> Self {
        Self {
            resolution,
            mt_reg: DEFAULT_MT_REG,
        }
    }

    pub fn with_mt_reg(mut self, mt_reg: u8) -> Self {
        debug_assert!(
            (31..=254).contains(&mt_reg),
            "MT_REG must be between 31 and 254"
        );
        self.mt_reg = mt_reg;
        self
    }

    pub(crate) fn spec(&self) -> ResolutionSpec {
        match self.resolution {
            Resolution::High => ResolutionSpec {
                bits: 0b0000,
                delay_ms: 180 * self.mt_reg as u32 / DEFAULT_MT_REG as u32,
                lux_scale: 1.0,
            },
            Resolution::High2 => ResolutionSpec {
                bits: 0b0001,
                delay_ms: 180 * self.mt_reg as u32 / DEFAULT_MT_REG as u32,
                lux_scale: 0.5,
            },
            Resolution::Low => ResolutionSpec {
                bits: 0b0011,
                delay_ms: 24 * self.mt_reg as u32 / DEFAULT_MT_REG as u32,
                lux_scale: 1.0,
            },
        }
    }
}
