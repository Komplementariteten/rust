#[doc = "Register `RX_LPI_TRAN_CNTR` reader"]
pub type R = crate::R<RX_LPI_TRAN_CNTR_SPEC>;
#[doc = "Field `RXLPITRC` reader - RXLPITRC"]
pub type RXLPITRC_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - RXLPITRC"]
    #[inline(always)]
    pub fn rxlpitrc(&self) -> RXLPITRC_R {
        RXLPITRC_R::new(self.bits)
    }
}
#[doc = "Rx LPI transition counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_lpi_tran_cntr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RX_LPI_TRAN_CNTR_SPEC;
impl crate::RegisterSpec for RX_LPI_TRAN_CNTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_lpi_tran_cntr::R`](R) reader structure"]
impl crate::Readable for RX_LPI_TRAN_CNTR_SPEC {}
#[doc = "`reset()` method sets RX_LPI_TRAN_CNTR to value 0"]
impl crate::Resettable for RX_LPI_TRAN_CNTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
