#[doc = "Register `PECR` reader"]
pub type R = crate::R<PECR_SPEC>;
#[doc = "Field `PEC` reader - Packet error checking register This field contains the internal PEC when PECEN=1. The PEC is cleared by hardware when PE=0."]
pub type PEC_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Packet error checking register This field contains the internal PEC when PECEN=1. The PEC is cleared by hardware when PE=0."]
    #[inline(always)]
    pub fn pec(&self) -> PEC_R {
        PEC_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Access: No wait states\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pecr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PECR_SPEC;
impl crate::RegisterSpec for PECR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pecr::R`](R) reader structure"]
impl crate::Readable for PECR_SPEC {}
#[doc = "`reset()` method sets PECR to value 0"]
impl crate::Resettable for PECR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
