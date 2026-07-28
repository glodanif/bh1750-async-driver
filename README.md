# bh1750-driver

A `no_std`, [`embedded-hal-async`](https://crates.io/crates/embedded-hal-async) driver for the
ROHM **BH1750** ambient light sensor.

Bring your own bus — the driver is generic over any async `I2c` + `DelayNs`, so it runs on any
HAL that implements `embedded-hal-async` 1.0 (Embassy, `esp-hal`, …) with no chip-specific code.

## Features

- **Both measurement modes** — one-shot (measure on demand) and continuous (free-running reads).
- **Selectable resolution** — `High` (1 lx), `High2` (0.5 lx), and `Low` (4 lx, faster).
- **Type-state API** — the compiler enforces the `Uninitialized → PoweredOn → OneShot/Continuous`
  flow; you can't read before powering on and picking a mode, and any active state can `power_down`.
- **Optional `defmt`** — enable the `defmt` feature for `defmt::Format` on `Error` and `SensorData`.

## Usage

```rust
use bh1750_driver::{Address, Bh1750Device, Resolution};

let device = Bh1750Device::new(i2c, Address::AddrLow, delay);

let mut bh1750 = device
    .power_on()
    .await?
    .into_continuous(Resolution::High)
    .await?;

let data = bh1750.read().await?;
let lux = data.light_intensity_lux();
```

For single measurements, use one-shot mode instead:

```rust
use bh1750_driver::{Address, Bh1750Device, Resolution};

let mut bh1750 = device
    .power_on()
    .await?
    .into_one_shot(Resolution::High);

let data = bh1750.measure().await?; // triggers a conversion, waits, and reads
```

`SensorData` stores the raw sensor count (`raw_output`); call `light_intensity_lux()` when you
want the value in `f32` lux.

## Adding it as a dependency

```toml
[dependencies]
bh1750-driver = { git = "https://github.com/glodanif/bh1750-async-driver", tag = "v0.1.0" }
```

Enable defmt logging with `features = ["defmt"]`.

## Addressing

The `ADDR` pin selects the I2C address: tie it low for `Address::AddrLow` (`0x23`) or high for
`Address::AddrHigh` (`0x5C`). `Address::Custom(u8)` is available for non-standard wiring, and
expects a 7-bit address.

## Example

[`nrf52-example`](nrf52-example) is a flashable Embassy application for the nRF52 DK
(probe-rs + defmt over RTT) that exercises the driver on real hardware.

## License

Licensed under the [MIT license](LICENSE).
