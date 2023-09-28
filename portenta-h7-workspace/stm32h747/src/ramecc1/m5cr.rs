#[doc = "Register `M5CR` reader"]
pub type R = crate::R<M5CR_SPEC>;
#[doc = "Field `FEC` reader - Failing error code"]
pub type FEC_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Failing error code"]
    #[inline(always)]
    pub fn fec(&self) -> FEC_R {
        FEC_R::new(self.bits)
    }
}
#[doc = "RAMECC monitor x configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m5cr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct M5CR_SPEC;
impl crate::RegisterSpec for M5CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`m5cr::R`](R) reader structure"]
impl crate::Readable for M5CR_SPEC {}
#[doc = "`reset()` method sets M5CR to value 0"]
impl crate::Resettable for M5CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
