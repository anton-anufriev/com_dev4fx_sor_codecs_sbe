#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum CancelRejectResponseTo {
    ORDER_CANCEL_REQUEST = 70_u8, 
    ORDER_CANCEL_REPLACE_REQUEST = 71_u8, 
    NullVal = 0_u8, 
}
impl Default for CancelRejectResponseTo {
    #[inline]
    fn default() -> Self { CancelRejectResponseTo::NullVal }
}
impl From<u8> for CancelRejectResponseTo {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            70_u8 => Self::ORDER_CANCEL_REQUEST, 
            71_u8 => Self::ORDER_CANCEL_REPLACE_REQUEST, 
            _ => Self::NullVal,
        }
    }
}
