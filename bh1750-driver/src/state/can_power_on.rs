use crate::command::Command;
use crate::state::powered_on::PoweredOn;
use crate::{Bh1750Device, Error};
use embedded_hal_async::delay::DelayNs;
use embedded_hal_async::i2c::I2c;
use crate::state::sealed::Sealed;

/// Trait for states that can transition to the PoweredOn state
pub trait CanPowerOn: Sealed {}

impl<T, D, S> Bh1750Device<T, D, S>
where
    T: I2c,
    D: DelayNs,
    S: CanPowerOn,
{

    /// Transition to the PoweredOn state
    ///
    /// # Errors
    ///
    /// Returns [Error::Bus] if writing the power-on command over I2C fails
    pub async fn power_on(mut self) -> Result<Bh1750Device<T, D, PoweredOn>, Error<T::Error>> {
        self.i2c_bus
            .write(self.address, &[Command::PowerOn.opcode()])
            .await
            .map_err(Error::Bus)?;
        self.delay.delay_ms(10).await;
        Ok(self.with_state(PoweredOn))
    }
}
