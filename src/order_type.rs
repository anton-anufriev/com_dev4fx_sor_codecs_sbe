#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum OrderType {
    MARKET = 49_u8, 
    LIMIT = 50_u8, 
    NullVal = 0_u8, 
}
impl Default for OrderType {
    #[inline]
    fn default() -> Self { OrderType::NullVal }
}
impl From<u8> for OrderType {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            49_u8 => Self::MARKET, 
            50_u8 => Self::LIMIT, 
            _ => Self::NullVal,
        }
    }
}
