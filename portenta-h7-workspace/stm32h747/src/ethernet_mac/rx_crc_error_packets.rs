#[doc = "Register `RX_CRC_ERROR_PACKETS` reader"]
pub type R = crate::R<RX_CRC_ERROR_PACKETS_SPEC>;
#[doc = "Field `RXCRCERR` reader - RXCRCERR"]
pub type RXCRCERR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - RXCRCERR"]
    #[inline(always)]
    pub fn rxcrcerr(&self) -> RXCRCERR_R {
        RXCRCERR_R::new(self.bits)
    }
}
#[doc = "Rx CRC error packets register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_crc_error_packets::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RX_CRC_ERROR_PACKETS_SPEC;
impl crate::RegisterSpec for RX_CRC_ERROR_PACKETS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_crc_error_packets::R`](R) reader structure"]
impl crate::Readable for RX_CRC_ERROR_PACKETS_SPEC {}
#[doc = "`reset()` method sets RX_CRC_ERROR_PACKETS to value 0"]
impl crate::Resettable for RX_CRC_ERROR_PACKETS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
