use crate::state::can_power_down::CanPowerDown;
use crate::state::can_power_on::CanPowerOn;
use crate::state::sealed::Sealed;
use crate::{Address, Bh1750Device};
use embedded_hal_async::delay::DelayNs;
use embedded_hal_async::i2c::I2c;

/// Uninitialized state
pub struct Uninitialized;

impl Sealed for Uninitialized {}
impl CanPowerOn for Uninitialized {}
impl CanPowerDown for Uninitialized {}

impl<T, D> Bh1750Device<T, D, Uninitialized>
where
    T: I2c,
    D: DelayNs,
{
    /// Create a new instance of the driver in the Uninitialized state
    pub fn new(i2c_bus: T, address: Address, delay: D) -> Self {
        let address = match address {
            Address::AddrLow => 0x23,
            Address::AddrHigh => 0x5C,
            Address::Custom(addr) => {
                debug_assert!(
                    addr <= 0x77,
                    "I2C 7-bit address must be 0x08..=0x77; did you pass the 8-bit R/W form?"
                );
                addr
            }
        };
        Self {
            i2c_bus,
            address,
            delay,
            state: Uninitialized,
        }
    }
}
