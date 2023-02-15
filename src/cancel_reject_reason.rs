#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum CancelRejectReason {
    TOO_LATE_TO_CANCEL = 48_u8, 
    UNKNOWN_ORDER = 49_u8, 
    BROKER_OPTION = 50_u8, 
    ALREADY_IN_PENDING = 51_u8, 
    NullVal = 0_u8, 
}
impl Default for CancelRejectReason {
    #[inline]
    fn default() -> Self { CancelRejectReason::NullVal }
}
impl From<u8> for CancelRejectReason {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            48_u8 => Self::TOO_LATE_TO_CANCEL, 
            49_u8 => Self::UNKNOWN_ORDER, 
            50_u8 => Self::BROKER_OPTION, 
            51_u8 => Self::ALREADY_IN_PENDING, 
            _ => Self::NullVal,
        }
    }
}
