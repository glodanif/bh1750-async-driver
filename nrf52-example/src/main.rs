#![no_std]
#![no_main]

use bh1750_driver::{Address, Bh1750Device, Config as Bh1750Config, Resolution};
use embassy_executor::Spawner;
use embassy_nrf::twim::{Config, InterruptHandler, Twim};
use embassy_nrf::{bind_interrupts, peripherals};
use embassy_time::{Delay, Timer};
#[allow(unused)]
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    let peri = embassy_nrf::init(Default::default());

    let mut rx_ram_buffer: [u8; 16] = [0u8; 16];
    let i2c = Twim::new(
        peri.TWISPI0,
        Irqs,
        peri.P0_26,
        peri.P0_27,
        Config::default(),
        &mut rx_ram_buffer,
    );

    let new_bh1750 = Bh1750Device::new(i2c, Address::AddrLow, Delay);
    let mut bh1750 = new_bh1750
        .power_on()
        .await
        .expect("Failed to power on bh1750")
        .into_continuous(Bh1750Config::new(Resolution::High2).with_mt_reg(69))
        .await
        .expect("Failed to start continuous bh1750");

    Timer::after_millis(500).await;

    loop {
        Timer::after_secs(1).await;
        let sensor_data = bh1750.read().await;
        match sensor_data {
            Ok(data) => {
                defmt::info!("BH1750 read: LI {}lx", data.light_intensity_lux(),);
            }
            Err(e) => {
                defmt::error!("Failed to read BH1750: {}", e);
            }
        }
    }
}

bind_interrupts!(struct Irqs {
      TWISPI0 => InterruptHandler<peripherals::TWISPI0>;
});
