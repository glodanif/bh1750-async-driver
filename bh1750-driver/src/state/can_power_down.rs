use crate::command::Command;
use crate::state::powered_down::PoweredDown;
use crate::{Bh1750Device, Error};
use embedded_hal_async::delay::DelayNs;
use embedded_hal_async::i2c::I2c;
use crate::state::sealed::Sealed;

/// Trait for states that can transition to the PoweredDown state
pub trait CanPowerDown: Sealed {}

impl<T, D, S> Bh1750Device<T, D, S>
where
    T: I2c,
    D: DelayNs,
    S: CanPowerDown,
{

    /// Transition to the PoweredDown state
    ///
    /// # Errors
    ///
    /// Returns [Error::Bus] if writing the power-down command over I2C fails
    pub async fn power_down(mut self) -> Result<Bh1750Device<T, D, PoweredDown>, Error<T::Error>> {
        self.i2c_bus
            .write(self.address, &[Command::PowerDown.opcode()])
            .await
            .map_err(Error::Bus)?;
        Ok(self.with_state(PoweredDown))
    }
}
