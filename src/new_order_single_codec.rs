use crate::*;

pub use encoder::*;
pub use decoder::*;

pub const SBE_BLOCK_LENGTH: u16 = 73;
pub const SBE_TEMPLATE_ID: u16 = 7;
pub const SBE_SCHEMA_ID: u16 = 1;
pub const SBE_SCHEMA_VERSION: u16 = 0;

pub mod encoder {
    use super::*;

    #[derive(Debug, Default)]
    pub struct NewOrderSingleEncoder<'a> {
        buf: WriteBuf<'a>,
        initial_offset: usize,
        offset: usize,
        limit: usize,
    }

    impl<'a> Writer<'a> for NewOrderSingleEncoder<'a> {
        #[inline]
        fn get_buf_mut(&mut self) -> &mut WriteBuf<'a> {
            &mut self.buf
        }
    }

    impl<'a> Encoder<'a> for NewOrderSingleEncoder<'a> {
        #[inline]
        fn get_limit(&self) -> usize {
            self.limit
        }

        #[inline]
        fn set_limit(&mut self, limit: usize) {
            self.limit = limit;
        }
    }

    impl<'a> NewOrderSingleEncoder<'a> {
        pub fn wrap(mut self, buf: WriteBuf<'a>, offset: usize) -> Self {
            let limit = offset + SBE_BLOCK_LENGTH as usize;
            self.buf = buf;
            self.initial_offset = offset;
            self.offset = offset;
            self.limit = limit;
            self
        }

        #[inline]
        pub fn encoded_length(&self) -> usize {
            self.limit - self.offset
        }

        pub fn header(self, offset: usize) -> MessageHeaderEncoder<Self> {
            let mut header = MessageHeaderEncoder::default().wrap(self, offset);
            header.block_length(SBE_BLOCK_LENGTH);
            header.template_id(SBE_TEMPLATE_ID);
            header.schema_id(SBE_SCHEMA_ID);
            header.version(SBE_SCHEMA_VERSION);
            header
        }

        /// primitive field 'clOrdId'
        /// - min value: 0
        /// - max value: -2
        /// - null value: -1
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 0
        /// - encodedLength: 8
        #[inline]
        pub fn cl_ord_id(&mut self, value: u64) {
            let offset = self.offset;
            self.get_buf_mut().put_u64_at(offset, value);
        }

        /// primitive field 'clOrdLinkId'
        /// - min value: 0
        /// - max value: -2
        /// - null value: -1
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 8
        /// - encodedLength: 8
        #[inline]
        pub fn cl_ord_link_id(&mut self, value: u64) {
            let offset = self.offset + 8;
            self.get_buf_mut().put_u64_at(offset, value);
        }

        /// primitive field 'instrumentId'
        /// - min value: 0
        /// - max value: 4294967294
        /// - null value: 4294967295
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 16
        /// - encodedLength: 4
        #[inline]
        pub fn instrument_id(&mut self, value: u32) {
            let offset = self.offset + 16;
            self.get_buf_mut().put_u32_at(offset, value);
        }

        /// primitive field 'targetCompId'
        /// - min value: 0
        /// - max value: 4294967294
        /// - null value: 4294967295
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 20
        /// - encodedLength: 4
        #[inline]
        pub fn target_comp_id(&mut self, value: u32) {
            let offset = self.offset + 20;
            self.get_buf_mut().put_u32_at(offset, value);
        }

        /// primitive field 'sourceCompId'
        /// - min value: 0
        /// - max value: 4294967294
        /// - null value: 4294967295
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 24
        /// - encodedLength: 4
        #[inline]
        pub fn source_comp_id(&mut self, value: u32) {
            let offset = self.offset + 24;
            self.get_buf_mut().put_u32_at(offset, value);
        }

        /// REQUIRED enum
        #[inline]
        pub fn side(&mut self, value: OrderSide) {
            let offset = self.offset + 28;
            self.get_buf_mut().put_u8_at(offset, value as u8)
        }

        /// REQUIRED enum
        #[inline]
        pub fn order_type(&mut self, value: OrderType) {
            let offset = self.offset + 29;
            self.get_buf_mut().put_u8_at(offset, value as u8)
        }

        /// primitive field 'price'
        /// - min value: 4.9E-324
        /// - max value: 1.7976931348623157E308
        /// - null value: NaN
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 30
        /// - encodedLength: 8
        #[inline]
        pub fn price(&mut self, value: f64) {
            let offset = self.offset + 30;
            self.get_buf_mut().put_f64_at(offset, value);
        }

        /// primitive field 'orderQty'
        /// - min value: 4.9E-324
        /// - max value: 1.7976931348623157E308
        /// - null value: NaN
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 38
        /// - encodedLength: 8
        #[inline]
        pub fn order_qty(&mut self, value: f64) {
            let offset = self.offset + 38;
            self.get_buf_mut().put_f64_at(offset, value);
        }

        /// REQUIRED enum
        #[inline]
        pub fn time_in_force(&mut self, value: TimeInForce) {
            let offset = self.offset + 46;
            self.get_buf_mut().put_u8_at(offset, value as u8)
        }

        /// primitive field 'transactTime'
        /// - min value: 0
        /// - max value: -2
        /// - null value: -1
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 47
        /// - encodedLength: 8
        #[inline]
        pub fn transact_time(&mut self, value: u64) {
            let offset = self.offset + 47;
            self.get_buf_mut().put_u64_at(offset, value);
        }

        /// primitive field 'effectiveTime'
        /// - min value: 0
        /// - max value: -2
        /// - null value: -1
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 55
        /// - encodedLength: 8
        #[inline]
        pub fn effective_time(&mut self, value: u64) {
            let offset = self.offset + 55;
            self.get_buf_mut().put_u64_at(offset, value);
        }

        /// primitive field 'expireTime'
        /// - min value: 0
        /// - max value: -2
        /// - null value: -1
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 63
        /// - encodedLength: 8
        #[inline]
        pub fn expire_time(&mut self, value: u64) {
            let offset = self.offset + 63;
            self.get_buf_mut().put_u64_at(offset, value);
        }

        /// REQUIRED enum
        #[inline]
        pub fn target_strategy(&mut self, value: Strategy) {
            let offset = self.offset + 71;
            self.get_buf_mut().put_u8_at(offset, value as u8)
        }

        /// REQUIRED enum
        #[inline]
        pub fn source_strategy(&mut self, value: Strategy) {
            let offset = self.offset + 72;
            self.get_buf_mut().put_u8_at(offset, value as u8)
        }

    }

} // end encoder

pub mod decoder {
    use super::*;

    #[derive(Debug, Default)]
    pub struct NewOrderSingleDecoder<'a> {
        buf: ReadBuf<'a>,
        initial_offset: usize,
        offset: usize,
        limit: usize,
        pub acting_block_length: u16,
        pub acting_version: u16,
    }

    impl<'a> Reader<'a> for NewOrderSingleDecoder<'a> {
        #[inline]
        fn get_buf(&self) -> &ReadBuf<'a> {
            &self.buf
        }
    }

    impl<'a> Decoder<'a> for NewOrderSingleDecoder<'a> {
        #[inline]
        fn get_limit(&self) -> usize {
            self.limit
        }

        #[inline]
        fn set_limit(&mut self, limit: usize) {
            self.limit = limit;
        }
    }

    impl<'a> NewOrderSingleDecoder<'a> {
        pub fn wrap(
            mut self,
            buf: ReadBuf<'a>,
            offset: usize,
            acting_block_length: u16,
            acting_version: u16,
        ) -> Self {
            let limit = offset + acting_block_length as usize;
            self.buf = buf;
            self.initial_offset = offset;
            self.offset = offset;
            self.limit = limit;
            self.acting_block_length = acting_block_length;
            self.acting_version = acting_version;
            self
        }

        #[inline]
        pub fn encoded_length(&self) -> usize {
            self.limit - self.offset
        }

        pub fn header(self, mut header: MessageHeaderDecoder<ReadBuf<'a>>) -> Self {
            debug_assert_eq!(SBE_TEMPLATE_ID, header.template_id());
            let acting_block_length = header.block_length();
            let acting_version = header.version();

            self.wrap(
                header.parent().unwrap(),
                message_header_codec::ENCODED_LENGTH,
                acting_block_length,
                acting_version,
            )
        }

        /// primitive field - 'REQUIRED'
        #[inline]
        pub fn cl_ord_id(&self) -> u64 {
            self.get_buf().get_u64_at(self.offset)
        }

        /// primitive field - 'REQUIRED'
        #[inline]
        pub fn cl_ord_link_id(&self) -> u64 {
            self.get_buf().get_u64_at(self.offset + 8)
        }

        /// primitive field - 'REQUIRED'
        #[inline]
        pub fn instrument_id(&self) -> u32 {
            self.get_buf().get_u32_at(self.offset + 16)
        }

        /// primitive field - 'REQUIRED'
        #[inline]
        pub fn target_comp_id(&self) -> u32 {
            self.get_buf().get_u32_at(self.offset + 20)
        }

        /// primitive field - 'REQUIRED'
        #[inline]
        pub fn source_comp_id(&self) -> u32 {
            self.get_buf().get_u32_at(self.offset + 24)
        }

        /// REQUIRED enum
        #[inline]
        pub fn side(&self) -> OrderSide {
            self.get_buf().get_u8_at(self.offset + 28).into()
        }

        /// REQUIRED enum
        #[inline]
        pub fn order_type(&self) -> OrderType {
            self.get_buf().get_u8_at(self.offset + 29).into()
        }

        /// primitive field - 'REQUIRED'
        #[inline]
        pub fn price(&self) -> f64 {
            self.get_buf().get_f64_at(self.offset + 30)
        }

        /// primitive field - 'REQUIRED'
        #[inline]
        pub fn order_qty(&self) -> f64 {
            self.get_buf().get_f64_at(self.offset + 38)
        }

        /// REQUIRED enum
        #[inline]
        pub fn time_in_force(&self) -> TimeInForce {
            self.get_buf().get_u8_at(self.offset + 46).into()
        }

        /// primitive field - 'REQUIRED'
        #[inline]
        pub fn transact_time(&self) -> u64 {
            self.get_buf().get_u64_at(self.offset + 47)
        }

        /// primitive field - 'REQUIRED'
        #[inline]
        pub fn effective_time(&self) -> u64 {
            self.get_buf().get_u64_at(self.offset + 55)
        }

        /// primitive field - 'REQUIRED'
        #[inline]
        pub fn expire_time(&self) -> u64 {
            self.get_buf().get_u64_at(self.offset + 63)
        }

        /// REQUIRED enum
        #[inline]
        pub fn target_strategy(&self) -> Strategy {
            self.get_buf().get_u8_at(self.offset + 71).into()
        }

        /// REQUIRED enum
        #[inline]
        pub fn source_strategy(&self) -> Strategy {
            self.get_buf().get_u8_at(self.offset + 72).into()
        }

    }

} // end decoder

