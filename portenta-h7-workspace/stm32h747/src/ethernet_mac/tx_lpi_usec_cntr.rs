#[doc = "Register `TX_LPI_USEC_CNTR` reader"]
pub type R = crate::R<TX_LPI_USEC_CNTR_SPEC>;
#[doc = "Field `TXLPIUSC` reader - TXLPIUSC"]
pub type TXLPIUSC_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - TXLPIUSC"]
    #[inline(always)]
    pub fn txlpiusc(&self) -> TXLPIUSC_R {
        TXLPIUSC_R::new(self.bits)
    }
}
#[doc = "Tx LPI microsecond timer register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_lpi_usec_cntr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TX_LPI_USEC_CNTR_SPEC;
impl crate::RegisterSpec for TX_LPI_USEC_CNTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_lpi_usec_cntr::R`](R) reader structure"]
impl crate::Readable for TX_LPI_USEC_CNTR_SPEC {}
#[doc = "`reset()` method sets TX_LPI_USEC_CNTR to value 0"]
impl crate::Resettable for TX_LPI_USEC_CNTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
