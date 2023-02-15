#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum TimeInForce {
    DAY = 48_u8, 
    GTC = 49_u8, 
    IOC = 51_u8, 
    FOK = 52_u8, 
    GTD = 54_u8, 
    NullVal = 0_u8, 
}
impl Default for TimeInForce {
    #[inline]
    fn default() -> Self { TimeInForce::NullVal }
}
impl From<u8> for TimeInForce {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            48_u8 => Self::DAY, 
            49_u8 => Self::GTC, 
            51_u8 => Self::IOC, 
            52_u8 => Self::FOK, 
            54_u8 => Self::GTD, 
            _ => Self::NullVal,
        }
    }
}
