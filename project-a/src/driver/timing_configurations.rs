use fugit::{MicrosDurationU32, MillisDurationU32};

pub const T_PPDP: MicrosDurationU32 = MicrosDurationU32::micros(5);
pub const T_HLD0: MicrosDurationU32 = MicrosDurationU32::micros(5);
pub const T_SET: MicrosDurationU32 = MicrosDurationU32::nanos(100);
pub const T_HLD1: MicrosDurationU32 = MicrosDurationU32::nanos(100);
pub const T_DLY2: MicrosDurationU32 = MicrosDurationU32::micros(1);
pub const T_ERA: MillisDurationU32 = MillisDurationU32::millis(10);
pub const T_PROG: MillisDurationU32 = MillisDurationU32::millis(2);
pub const T_DIS: MicrosDurationU32 = MicrosDurationU32::micros(100);
pub const T_RESET: MillisDurationU32 = MillisDurationU32::millis(10);
