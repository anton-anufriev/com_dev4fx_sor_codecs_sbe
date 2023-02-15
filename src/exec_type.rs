#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum ExecType {
    NEW = 48_u8, 
    PARTIAL_FILL = 49_u8, 
    FILL = 50_u8, 
    DONE_FOR_DAY = 51_u8, 
    CANCELLED = 52_u8, 
    REPLACED = 53_u8, 
    PENDING_CANCEL = 54_u8, 
    STOPPED = 55_u8, 
    REJECTED = 56_u8, 
    SUSPENDED = 57_u8, 
    PENDING_NEW = 65_u8, 
    CALCULATED = 66_u8, 
    EXPIRED = 67_u8, 
    RESTATED = 68_u8, 
    PENDING_REPLACE = 69_u8, 
    NullVal = 0_u8, 
}
impl Default for ExecType {
    #[inline]
    fn default() -> Self { ExecType::NullVal }
}
impl From<u8> for ExecType {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            48_u8 => Self::NEW, 
            49_u8 => Self::PARTIAL_FILL, 
            50_u8 => Self::FILL, 
            51_u8 => Self::DONE_FOR_DAY, 
            52_u8 => Self::CANCELLED, 
            53_u8 => Self::REPLACED, 
            54_u8 => Self::PENDING_CANCEL, 
            55_u8 => Self::STOPPED, 
            56_u8 => Self::REJECTED, 
            57_u8 => Self::SUSPENDED, 
            65_u8 => Self::PENDING_NEW, 
            66_u8 => Self::CALCULATED, 
            67_u8 => Self::EXPIRED, 
            68_u8 => Self::RESTATED, 
            69_u8 => Self::PENDING_REPLACE, 
            _ => Self::NullVal,
        }
    }
}
