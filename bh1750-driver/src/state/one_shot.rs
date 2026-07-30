use crate::config::Config;
use crate::parameters::Mode;
use crate::sensor_data::SensorData;
use crate::state::can_power_down::CanPowerDown;
use crate::state::sealed::Sealed;
use crate::{Bh1750Device, Error};
use embedded_hal_async::delay::DelayNs;
use embedded_hal_async::i2c::I2c;

/// One-shot measurement state
pub struct OneShot {
    pub(crate) config: Config,
}

impl Sealed for OneShot {}
impl CanPowerDown for OneShot {}

impl<T, D> Bh1750Device<T, D, OneShot>
where
    T: I2c,
    D: DelayNs,
{
    /// Measure the light level
    ///
    /// # Errors
    ///
    /// Returns [Error::Bus] if writing the one-shot measurement command or
    /// reading the measurement result back over I2C fails
    pub async fn measure(&mut self) -> Result<SensorData, Error<T::Error>> {
        let spec = self.state.config.spec();
        let mode = (Mode::OneShot as u8) << 4 | spec.bits;
        self.i2c_bus
            .write(self.address, &[mode])
            .await
            .map_err(Error::Bus)?;
        self.delay.delay_ms(spec.delay_ms).await;
        let mut data_out = [0u8; 2];
        self.i2c_bus
            .read(self.address, &mut data_out)
            .await
            .map_err(Error::Bus)?;
        Ok(SensorData::from_be_bytes(
            data_out,
            self.state.config.mt_reg,
            spec.lux_scale,
        ))
    }
}
