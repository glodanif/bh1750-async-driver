#![cfg_attr(not(test), no_std)]
#![warn(missing_docs)]

//! # BH1750 async driver
//!
//! A `no_std` driver for the ROHM BH1750 ambient light sensor, built on
//! `embedded-hal-async`. Bring your own bus — generic over any `I2c` +
//! `DelayNs`.
//!
//! The driver is type-state: the sensor's power and measurement mode live in
//! the type, so only transitions valid for the current state compile. The flow
//! is [Uninitialized] -> [PoweredOn] -> [OneShot] / [Continuous], and most
//! states can [power_down](Bh1750Device::power_down) back to [PoweredDown].
//!
//! From [PoweredOn], pick a mode with
//! [into_one_shot](Bh1750Device::into_one_shot) for on-demand
//! [measure](Bh1750Device::measure)s, or
//! [into_continuous](Bh1750Device::into_continuous) for repeated
//! [read](Bh1750Device::read)s.
//!
//! ### Resolution
//!
//! Each measurement mode takes a [Resolution]: [High](Resolution::High) (1 lx),
//! [High2](Resolution::High2) (0.5 lx), or [Low](Resolution::Low) (4 lx,
//! faster). [SensorData::light_intensity_lux] converts the raw count to lux.
//!
//! ### Continuous mode example
//!
//! ```no_run
//! # use bh1750_driver::{Address, Bh1750Device, Resolution};
//! # use embedded_hal_mock::eh1::i2c::Mock as I2c;
//! # use embedded_hal_mock::eh1::delay::NoopDelay as Delay;
//! # async fn example() {
//! # let i2c = I2c::new(&[]);
//! # let delay = Delay::new();
//! let device = Bh1750Device::new(i2c, Address::AddrLow, delay);
//! let mut bh1750 = device
//!     .power_on()
//!     .await
//!     .unwrap()
//!     .into_continuous(Resolution::High)
//!     .await
//!     .unwrap();
//! let data = bh1750.read().await.unwrap();
//! let lux = data.light_intensity_lux();
//! # }
//! ```
//!
//! ### One-shot mode example
//!
//! ```no_run
//! # use bh1750_driver::{Address, Bh1750Device, Resolution};
//! # use embedded_hal_mock::eh1::i2c::Mock as I2c;
//! # use embedded_hal_mock::eh1::delay::NoopDelay as Delay;
//! # async fn example() {
//! # let i2c = I2c::new(&[]);
//! # let delay = Delay::new();
//! let device = Bh1750Device::new(i2c, Address::AddrHigh, delay);
//! let mut bh1750 = device
//!     .power_on()
//!     .await
//!     .unwrap()
//!     .into_one_shot(Resolution::High);
//! let data = bh1750.measure().await.unwrap();
//! # }
//! ```
//!
//! ### Optional features
//!
//! Enable the `defmt` feature for `defmt::Format` on [Error] and [SensorData].

mod command;
mod parameters;
mod sensor_data;
mod state;
mod config;

pub use crate::parameters::Resolution;
pub use crate::config::Config;
pub use crate::sensor_data::SensorData;
pub use crate::state::{Continuous, OneShot, PoweredDown, PoweredOn, Uninitialized};

use embedded_hal_async::delay::DelayNs;
use embedded_hal_async::i2c::I2c;

/// Number of counts per lux
pub const COUNTS_PER_LUX: f32 = 1.2;

/// I2C device address
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Clone, Copy)]
pub enum Address {
    /// 0x23
    AddrLow,
    /// 0x5C
    AddrHigh,
    /// Custom address
    Custom(u8),
}

/// I2C error
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug)]
pub enum Error<E> {
    /// I2C bus error; a failure while sending a command to or reading a
    /// measurement from the sensor over the I2C bus
    Bus(E),
}

/// The BH1750 driver. The type-state `S` tracks the sensor's current power and
/// measurement state.
pub struct Bh1750Device<T, D, S> {
    i2c_bus: T,
    address: u8,
    delay: D,
    state: S,
}

impl<T, D, S> Bh1750Device<T, D, S>
where
    T: I2c,
    D: DelayNs,
{
    fn with_state<S2>(self, state: S2) -> Bh1750Device<T, D, S2> {
        Bh1750Device {
            i2c_bus: self.i2c_bus,
            address: self.address,
            delay: self.delay,
            state,
        }
    }
}
