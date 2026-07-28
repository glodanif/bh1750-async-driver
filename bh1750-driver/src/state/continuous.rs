use crate::parameters::ResolutionSpec;
use crate::sensor_data::SensorData;
use crate::state::can_power_down::CanPowerDown;
use crate::{Bh1750Device, Error};
use embedded_hal_async::delay::DelayNs;
use embedded_hal_async::i2c::I2c;
use crate::command::Command;
use crate::state::sealed::Sealed;

/// Continuous measurement state
pub struct Continuous {
    pub(crate) spec: ResolutionSpec,
}

impl Sealed for Continuous {}
impl CanPowerDown for Continuous {}

impl<T, D> Bh1750Device<T, D, Continuous>
where
    T: I2c,
    D: DelayNs,
{
    
    /// Read the light level
    ///
    /// # Errors
    ///
    /// Returns [Error::Bus] if reading the measurement result over I2C fails
    pub async fn read(&mut self) -> Result<SensorData, Error<T::Error>> {
        let mut data_out = [0u8; 2];
        self.i2c_bus
            .read(self.address, &mut data_out)
            .await
            .map_err(Error::Bus)?;
        Ok(SensorData::from_be_bytes(
            data_out,
            self.state.spec.lux_scale,
        ))
    }

    /// Reset the sensor
    ///
    /// # Errors
    ///
    /// Returns [Error::Bus] if writing the reset command over I2C fails
    pub async fn reset(&mut self) -> Result<(), Error<T::Error>> {
        self.i2c_bus
            .write(self.address, &[Command::Reset.opcode()])
            .await
            .map_err(Error::Bus)?;
        Ok(())
    }
}
