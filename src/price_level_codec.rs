use crate::*;

pub use encoder::*;
pub use decoder::*;

pub const ENCODED_LENGTH: usize = 28;

pub mod encoder {
    use super::*;

    #[derive(Debug, Default)]
    pub struct PriceLevelEncoder<P> {
        parent: Option<P>,
        offset: usize,
    }

    impl<'a, P> Writer<'a> for PriceLevelEncoder<P> where P: Writer<'a> + Default {
        #[inline]
        fn get_buf_mut(&mut self) -> &mut WriteBuf<'a> {
            if let Some(parent) = self.parent.as_mut() {
                parent.get_buf_mut()
            } else {
                panic!("parent was None")
            }
        }
    }

    impl<'a, P> PriceLevelEncoder<P> where P: Writer<'a> + Default {
        pub fn wrap(mut self, parent: P, offset: usize) -> Self {
            self.parent = Some(parent);
            self.offset = offset;
            self
        }

        #[inline]
        pub fn parent(&mut self) -> SbeResult<P> {
            self.parent.take().ok_or(SbeErr::ParentNotSet)
        }

        /// primitive field 'id'
        /// - min value: 0
        /// - max value: 4294967294
        /// - null value: 4294967295
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 0
        /// - encodedLength: 4
        #[inline]
        pub fn id(&mut self, value: u32) {
            let offset = self.offset;
            self.get_buf_mut().put_u32_at(offset, value);
        }

        /// primitive field 'price'
        /// - min value: 4.9E-324
        /// - max value: 1.7976931348623157E308
        /// - null value: NaN
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 4
        /// - encodedLength: 8
        #[inline]
        pub fn price(&mut self, value: f64) {
            let offset = self.offset + 4;
            self.get_buf_mut().put_f64_at(offset, value);
        }

        /// primitive field 'leavesQty'
        /// - min value: 4.9E-324
        /// - max value: 1.7976931348623157E308
        /// - null value: NaN
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 12
        /// - encodedLength: 8
        #[inline]
        pub fn leaves_qty(&mut self, value: f64) {
            let offset = self.offset + 12;
            self.get_buf_mut().put_f64_at(offset, value);
        }

        /// primitive field 'transactTime'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 20
        /// - encodedLength: 8
        #[inline]
        pub fn transact_time(&mut self, value: i64) {
            let offset = self.offset + 20;
            self.get_buf_mut().put_i64_at(offset, value);
        }

    }
} // end encoder mod 

pub mod decoder {
    use super::*;

    #[derive(Debug, Default)]
    pub struct PriceLevelDecoder<P> {
        parent: Option<P>,
        offset: usize,
    }

    impl<'a, P> Reader<'a> for PriceLevelDecoder<P> where P: Reader<'a> + Default {
        #[inline]
        fn get_buf(&self) -> &ReadBuf<'a> {
            self.parent.as_ref().expect("parent missing").get_buf()
        }
    }

    impl<'a, P> PriceLevelDecoder<P> where P: Reader<'a> + Default {
        pub fn wrap(mut self, parent: P, offset: usize) -> Self {
            self.parent = Some(parent);
            self.offset = offset;
            self
        }

        #[inline]
        pub fn parent(&mut self) -> SbeResult<P> {
            self.parent.take().ok_or(SbeErr::ParentNotSet)
        }

        /// primitive field - 'REQUIRED'
        #[inline]
        pub fn id(&self) -> u32 {
            self.get_buf().get_u32_at(self.offset)
        }

        /// primitive field - 'REQUIRED'
        #[inline]
        pub fn price(&self) -> f64 {
            self.get_buf().get_f64_at(self.offset + 4)
        }

        /// primitive field - 'REQUIRED'
        #[inline]
        pub fn leaves_qty(&self) -> f64 {
            self.get_buf().get_f64_at(self.offset + 12)
        }

        /// primitive field - 'REQUIRED'
        #[inline]
        pub fn transact_time(&self) -> i64 {
            self.get_buf().get_i64_at(self.offset + 20)
        }

    }
} // end decoder mod 
