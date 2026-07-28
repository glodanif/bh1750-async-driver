use crate::state::can_power_on::CanPowerOn;
use crate::state::sealed::Sealed;

/// Powered down state
pub struct PoweredDown;

impl Sealed for PoweredDown {}
impl CanPowerOn for PoweredDown {}
