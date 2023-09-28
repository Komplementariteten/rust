#[doc = "Register `MACTxTSSNR` reader"]
pub type R = crate::R<MACTX_TSSNR_SPEC>;
#[doc = "Field `TXTSSLO` reader - TXTSSLO"]
pub type TXTSSLO_R = crate::FieldReader<u32>;
#[doc = "Field `TXTSSMIS` reader - TXTSSMIS"]
pub type TXTSSMIS_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:30 - TXTSSLO"]
    #[inline(always)]
    pub fn txtsslo(&self) -> TXTSSLO_R {
        TXTSSLO_R::new(self.bits & 0x7fff_ffff)
    }
    #[doc = "Bit 31 - TXTSSMIS"]
    #[inline(always)]
    pub fn txtssmis(&self) -> TXTSSMIS_R {
        TXTSSMIS_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Tx timestamp status nanoseconds register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mactx_tssnr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACTX_TSSNR_SPEC;
impl crate::RegisterSpec for MACTX_TSSNR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mactx_tssnr::R`](R) reader structure"]
impl crate::Readable for MACTX_TSSNR_SPEC {}
#[doc = "`reset()` method sets MACTxTSSNR to value 0"]
impl crate::Resettable for MACTX_TSSNR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
