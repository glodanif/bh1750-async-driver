use crate::COUNTS_PER_LUX;

/// Represents the light intensity data from the sensor
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, PartialEq)]
pub struct SensorData {
    /// Raw output from the sensor
    pub raw_output: u16,
    pub(crate) lux_scale: f32,
}

impl SensorData {
    pub(crate) fn from_be_bytes(bytes: [u8; 2], lux_scale: f32) -> Self {
        Self {
            raw_output: u16::from_be_bytes(bytes),
            lux_scale,
        }
    }

    /// Returns the light intensity in lux
    pub fn light_intensity_lux(&self) -> f32 {
        self.raw_output as f32 / COUNTS_PER_LUX * self.lux_scale
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: SensorData = SensorData {
        raw_output: 1000,
        lux_scale: 1.5,
    };

    #[test]
    fn lux_calculation() {
        assert!((DATA.light_intensity_lux() - 1250.0).abs() < 0.001);
    }
}
