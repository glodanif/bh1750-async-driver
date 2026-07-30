use crate::command::Command;
use crate::config::Config;
use crate::parameters::Mode;
use crate::state::can_power_down::CanPowerDown;
use crate::state::continuous::Continuous;
use crate::state::one_shot::OneShot;
use crate::state::sealed::Sealed;
use crate::{Bh1750Device, Error};
use embedded_hal_async::delay::DelayNs;
use embedded_hal_async::i2c::I2c;

/// Powered on state
pub struct PoweredOn;

impl Sealed for PoweredOn {}
impl CanPowerDown for PoweredOn {}

impl<T, D> Bh1750Device<T, D, PoweredOn>
where
    T: I2c,
    D: DelayNs,
{
    /// Transition to the OneShot state
    pub async fn into_one_shot(
        mut self,
        config: Config,
    ) -> Result<Bh1750Device<T, D, OneShot>, Error<T::Error>> {
        self.set_mt_reg(config.mt_reg).await?;
        Ok(self.with_state(OneShot { config }))
    }

    /// Transition to the Continuous state
    ///
    /// # Errors
    ///
    /// Returns [Error::Bus] if writing the continuous measurement-mode command
    /// over I2C fails
    pub async fn into_continuous(
        mut self,
        config: Config,
    ) -> Result<Bh1750Device<T, D, Continuous>, Error<T::Error>> {
        let spec = config.spec();
        let mode = (Mode::Continuous as u8) << 4 | spec.bits;
        self.set_mt_reg(config.mt_reg).await?;
        self.i2c_bus
            .write(self.address, &[mode])
            .await
            .map_err(Error::Bus)?;
        self.delay.delay_ms(spec.delay_ms).await;
        Ok(self.with_state(Continuous { config }))
    }

    async fn set_mt_reg(&mut self, mt_reg: u8) -> Result<(), Error<T::Error>> {
        self.i2c_bus
            .write(
                self.address,
                &[Command::MeasurementTimeHi.opcode() | mt_reg >> 5],
            )
            .await
            .map_err(Error::Bus)?;
        self.i2c_bus
            .write(
                self.address,
                &[Command::MeasurementTimeLo.opcode() | mt_reg & 0x1F],
            )
            .await
            .map_err(Error::Bus)?;
        Ok(())
    }
}
