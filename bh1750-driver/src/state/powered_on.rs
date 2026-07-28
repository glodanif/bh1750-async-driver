use crate::parameters::{Mode, Resolution};
use crate::state::can_power_down::CanPowerDown;
use crate::state::continuous::Continuous;
use crate::state::one_shot::OneShot;
use crate::{Bh1750Device, Error};
use embedded_hal_async::delay::DelayNs;
use embedded_hal_async::i2c::I2c;
use crate::state::sealed::Sealed;

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
    pub fn into_one_shot(self, resolution: Resolution) -> Bh1750Device<T, D, OneShot> {
        self.with_state(OneShot { spec: resolution.spec() })
    }

    /// Transition to the Continuous state
    ///
    /// # Errors
    ///
    /// Returns [Error::Bus] if writing the continuous measurement-mode command
    /// over I2C fails
    pub async fn into_continuous(
        mut self,
        resolution: Resolution,
    ) -> Result<Bh1750Device<T, D, Continuous>, Error<T::Error>> {
        let spec = resolution.spec();
        let mode = (Mode::Continuous as u8) << 4 | spec.bits;
        self.i2c_bus
            .write(self.address, &[mode])
            .await
            .map_err(Error::Bus)?;
        self.delay.delay_ms(spec.delay_ms).await;
        Ok(self.with_state(Continuous { spec }))
    }
}
