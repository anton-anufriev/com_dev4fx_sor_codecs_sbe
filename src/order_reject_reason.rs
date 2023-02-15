#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum OrderRejectReason {
    BROKER_OPTION = 48_u8, 
    UNKNOWN_SYMBOL = 49_u8, 
    EXCHANGE_CLOSED = 50_u8, 
    ORDER_EXCEEDS_LIMIT = 51_u8, 
    TOO_LATE_TO_ENTER = 52_u8, 
    UNKNOWN_ORDER = 53_u8, 
    DUPLICATE_ORDER = 54_u8, 
    STALE_ORDER = 56_u8, 
    NullVal = 0_u8, 
}
impl Default for OrderRejectReason {
    #[inline]
    fn default() -> Self { OrderRejectReason::NullVal }
}
impl From<u8> for OrderRejectReason {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            48_u8 => Self::BROKER_OPTION, 
            49_u8 => Self::UNKNOWN_SYMBOL, 
            50_u8 => Self::EXCHANGE_CLOSED, 
            51_u8 => Self::ORDER_EXCEEDS_LIMIT, 
            52_u8 => Self::TOO_LATE_TO_ENTER, 
            53_u8 => Self::UNKNOWN_ORDER, 
            54_u8 => Self::DUPLICATE_ORDER, 
            56_u8 => Self::STALE_ORDER, 
            _ => Self::NullVal,
        }
    }
}
