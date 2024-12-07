#![allow(clippy::cast_sign_loss)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_precision_loss)]

pub const ONE_BIT: usize = 1;
pub const TWO_BITS: usize = 2;
pub const THREE_BITS: usize = 3;
pub const FOUR_BITS: usize = 4;
pub const FIVE_BITS: usize = 5;
pub const SIX_BITS: usize = 6;
pub const SEVEN_BITS: usize = 7;
pub const EIGHT_BITS: usize = 8;
pub const NINE_BITS: usize = 9;
pub const TEN_BITS: usize = 10;
pub const ELEVEN_BITS: usize = 11;
pub const TWELVE_BITS: usize = 12;
pub const THIRTEEN_BITS: usize = 13;
pub const FOURTEEN_BITS: usize = 14;
pub const FIFTEEN_BITS: usize = 15;
pub const SIXTEEN_BITS: usize = 16;
pub const SEVENTEEN_BITS: usize = 17;
pub const TWENTY_BITS: usize = 20;
pub const TWENTY_ONE_BITS: usize = 21;
pub const TWENTY_FOUR_BITS: usize = 24;
pub const TWENTY_SIX_BITS: usize = 26;
pub const TWENTY_EIGHT_BITS: usize = 28;
pub const THIRTY_BITS: usize = 30;
pub const THIRTY_ONE_BITS: usize = 31;
pub const THIRTY_TWO_BITS: usize = 32;
pub const THIRTY_NINE_BITS: usize = 39;
pub const FORTY_BITS: usize = 40;
pub const FORTY_EIGHT_BITS: usize = 48;
pub const SIXTY_FOUR_BITS: usize = 64;
pub const EIGHTY_SIX_BITS: usize = 86;
pub const NINETY_EIGHT_BITS: usize = 98;
pub const HUNDRED_TWELVE_BITS: usize = 112;
pub const HUNDRED_TWENTY_BITS: usize = 120;
pub const LEAST_SIGNIFICANT_BIT: u32 = 0x001;
pub const NANOSECONDS_PER_HOUR: u32 = 3600 * 1e6 as u32;
pub const CDIS_TIME_UNITS_PER_HOUR: u32 = (2 ^ 25) - 1;
pub const CDIS_NANOSECONDS_PER_TIME_UNIT: f32 =
    NANOSECONDS_PER_HOUR as f32 / CDIS_TIME_UNITS_PER_HOUR as f32;
pub const DIS_TIME_UNITS_PER_HOUR: u32 = (2 ^ 31) - 1;
pub const DIS_NANOSECONDS_PER_TIME_UNIT: f32 =
    NANOSECONDS_PER_HOUR as f32 / DIS_TIME_UNITS_PER_HOUR as f32;
pub const MTU_BYTES: usize = 1400; // Maximum Transmission Unit of the network
pub const MTU_BITS: usize = MTU_BYTES * EIGHT_BITS;
pub const DECIMETERS_IN_METER: f32 = 10f32;
pub const CENTIMETER_PER_METER: f32 = 100f32;
pub const RADIANS_SEC_TO_DEGREES_SEC: f32 = 180.0 / std::f32::consts::PI;
pub const CENTER_OF_EARTH_ALTITUDE: i32 = -8_388_608;
pub const ALTITUDE_CM_THRESHOLD: f64 = 8_388_608f64;
pub const MAX_VARIABLE_DATUM_LENGTH_BITS: u16 = 16_383;
pub const MAX_TRACK_JAM_NUMBER_OF_TARGETS: usize = 15;
