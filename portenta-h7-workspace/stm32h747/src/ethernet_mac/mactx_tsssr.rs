#[doc = "Register `MACTxTSSSR` reader"]
pub type R = crate::R<MACTX_TSSSR_SPEC>;
#[doc = "Field `TXTSSHI` reader - TXTSSHI"]
pub type TXTSSHI_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - TXTSSHI"]
    #[inline(always)]
    pub fn txtsshi(&self) -> TXTSSHI_R {
        TXTSSHI_R::new(self.bits)
    }
}
#[doc = "Tx timestamp status seconds register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mactx_tsssr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACTX_TSSSR_SPEC;
impl crate::RegisterSpec for MACTX_TSSSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mactx_tsssr::R`](R) reader structure"]
impl crate::Readable for MACTX_TSSSR_SPEC {}
#[doc = "`reset()` method sets MACTxTSSSR to value 0"]
impl crate::Resettable for MACTX_TSSSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
