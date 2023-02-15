#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum Strategy {
    SIMULATOR = 49_u8, 
    VENUE = 50_u8, 
    SWEEPER = 51_u8, 
    LIMIT_SNIPER = 52_u8, 
    STOP_LOSS = 53_u8, 
    TWAP = 54_u8, 
    VWAP = 55_u8, 
    NullVal = 0_u8, 
}
impl Default for Strategy {
    #[inline]
    fn default() -> Self { Strategy::NullVal }
}
impl From<u8> for Strategy {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            49_u8 => Self::SIMULATOR, 
            50_u8 => Self::VENUE, 
            51_u8 => Self::SWEEPER, 
            52_u8 => Self::LIMIT_SNIPER, 
            53_u8 => Self::STOP_LOSS, 
            54_u8 => Self::TWAP, 
            55_u8 => Self::VWAP, 
            _ => Self::NullVal,
        }
    }
}
