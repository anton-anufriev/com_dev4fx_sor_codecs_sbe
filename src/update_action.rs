#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum UpdateAction {
    NEW = 78_u8, 
    UPDATE = 85_u8, 
    DELETE = 68_u8, 
    NullVal = 0_u8, 
}
impl Default for UpdateAction {
    #[inline]
    fn default() -> Self { UpdateAction::NullVal }
}
impl From<u8> for UpdateAction {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            78_u8 => Self::NEW, 
            85_u8 => Self::UPDATE, 
            68_u8 => Self::DELETE, 
            _ => Self::NullVal,
        }
    }
}
