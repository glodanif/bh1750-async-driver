#[repr(u8)]
#[derive(Clone, Copy)]
pub(crate) enum Command {
    PowerDown = 0x00,
    PowerOn = 0x01,
    Reset = 0x07,
}

impl Command {
    pub const fn opcode(self) -> u8 {
        self as u8
    }
}
