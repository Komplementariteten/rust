#[doc = "Register `SIDR` reader"]
pub type R = crate::R<SIDR_SPEC>;
#[doc = "Field `SID` reader - Size identification"]
pub type SID_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Size identification"]
    #[inline(always)]
    pub fn sid(&self) -> SID_R {
        SID_R::new(self.bits)
    }
}
#[doc = "SPDIFRX size identification register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sidr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SIDR_SPEC;
impl crate::RegisterSpec for SIDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sidr::R`](R) reader structure"]
impl crate::Readable for SIDR_SPEC {}
#[doc = "`reset()` method sets SIDR to value 0xa3c5_dd01"]
impl crate::Resettable for SIDR_SPEC {
    const RESET_VALUE: Self::Ux = 0xa3c5_dd01;
}
