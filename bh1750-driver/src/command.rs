#[repr(u8)]
#[derive(Clone, Copy)]
pub(crate) enum Command {
    PowerDown = 0x00,
    PowerOn = 0x01,
    Reset = 0x07,
    MeasurementTimeHi = 0x40,
    MeasurementTimeLo = 0x60,
}

impl Command {
    pub const fn opcode(self) -> u8 {
        self as u8
    }
}
