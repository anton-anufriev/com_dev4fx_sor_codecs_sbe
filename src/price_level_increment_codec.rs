use crate::*;

pub use encoder::*;
pub use decoder::*;

pub const ENCODED_LENGTH: usize = 29;

pub mod encoder {
    use super::*;

    #[derive(Debug, Default)]
    pub struct PriceLevelIncrementEncoder<P> {
        parent: Option<P>,
        offset: usize,
    }

    impl<'a, P> Writer<'a> for PriceLevelIncrementEncoder<P> where P: Writer<'a> + Default {
        #[inline]
        fn get_buf_mut(&mut self) -> &mut WriteBuf<'a> {
            if let Some(parent) = self.parent.as_mut() {
                parent.get_buf_mut()
            } else {
                panic!("parent was None")
            }
        }
    }

    impl<'a, P> PriceLevelIncrementEncoder<P> where P: Writer<'a> + Default {
        pub fn wrap(mut self, parent: P, offset: usize) -> Self {
            self.parent = Some(parent);
            self.offset = offset;
            self
        }

        #[inline]
        pub fn parent(&mut self) -> SbeResult<P> {
            self.parent.take().ok_or(SbeErr::ParentNotSet)
        }

        /// COMPOSITE ENCODER
        #[inline]
        pub fn price_level_encoder(self) -> PriceLevelEncoder<Self> {
            let offset = self.offset;
            PriceLevelEncoder::default().wrap(self, offset)
        }

        /// REQUIRED enum
        #[inline]
        pub fn update_action(&mut self, value: UpdateAction) {
            let offset = self.offset + 28;
            self.get_buf_mut().put_u8_at(offset, value as u8)
        }

    }
} // end encoder mod 

pub mod decoder {
    use super::*;

    #[derive(Debug, Default)]
    pub struct PriceLevelIncrementDecoder<P> {
        parent: Option<P>,
        offset: usize,
    }

    impl<'a, P> Reader<'a> for PriceLevelIncrementDecoder<P> where P: Reader<'a> + Default {
        #[inline]
        fn get_buf(&self) -> &ReadBuf<'a> {
            self.parent.as_ref().expect("parent missing").get_buf()
        }
    }

    impl<'a, P> PriceLevelIncrementDecoder<P> where P: Reader<'a> + Default {
        pub fn wrap(mut self, parent: P, offset: usize) -> Self {
            self.parent = Some(parent);
            self.offset = offset;
            self
        }

        #[inline]
        pub fn parent(&mut self) -> SbeResult<P> {
            self.parent.take().ok_or(SbeErr::ParentNotSet)
        }

        /// COMPOSITE DECODER
        #[inline]
        pub fn price_level_decoder(self) -> PriceLevelDecoder<Self> {
            let offset = self.offset;
            PriceLevelDecoder::default().wrap(self, offset)
        }

        /// REQUIRED enum
        #[inline]
        pub fn update_action(&self) -> UpdateAction {
            self.get_buf().get_u8_at(self.offset + 28).into()
        }

    }
} // end decoder mod 
