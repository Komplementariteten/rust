#[doc = "Register `TX_LPI_TRAN_CNTR` reader"]
pub type R = crate::R<TX_LPI_TRAN_CNTR_SPEC>;
#[doc = "Field `TXLPITRC` reader - TXLPITRC"]
pub type TXLPITRC_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - TXLPITRC"]
    #[inline(always)]
    pub fn txlpitrc(&self) -> TXLPITRC_R {
        TXLPITRC_R::new(self.bits)
    }
}
#[doc = "Tx LPI transition counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_lpi_tran_cntr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TX_LPI_TRAN_CNTR_SPEC;
impl crate::RegisterSpec for TX_LPI_TRAN_CNTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_lpi_tran_cntr::R`](R) reader structure"]
impl crate::Readable for TX_LPI_TRAN_CNTR_SPEC {}
#[doc = "`reset()` method sets TX_LPI_TRAN_CNTR to value 0"]
impl crate::Resettable for TX_LPI_TRAN_CNTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
