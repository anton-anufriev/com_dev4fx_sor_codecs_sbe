#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum OrderSide {
    BUY = 49_u8, 
    SELL = 50_u8, 
    NullVal = 0_u8, 
}
impl Default for OrderSide {
    #[inline]
    fn default() -> Self { OrderSide::NullVal }
}
impl From<u8> for OrderSide {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            49_u8 => Self::BUY, 
            50_u8 => Self::SELL, 
            _ => Self::NullVal,
        }
    }
}
