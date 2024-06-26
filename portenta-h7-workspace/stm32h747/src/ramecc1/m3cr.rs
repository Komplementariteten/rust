#[doc = "Register `M3CR` reader"]
pub type R = crate::R<M3CR_SPEC>;
#[doc = "Field `FADD` reader - ECC error failing address"]
pub type FADD_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - ECC error failing address"]
    #[inline(always)]
    pub fn fadd(&self) -> FADD_R {
        FADD_R::new(self.bits)
    }
}
#[doc = "RAMECC monitor x configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m3cr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct M3CR_SPEC;
impl crate::RegisterSpec for M3CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`m3cr::R`](R) reader structure"]
impl crate::Readable for M3CR_SPEC {}
#[doc = "`reset()` method sets M3CR to value 0"]
impl crate::Resettable for M3CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
