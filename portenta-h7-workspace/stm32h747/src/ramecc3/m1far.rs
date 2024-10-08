#[doc = "Register `M1FAR` reader"]
pub type R = crate::R<M1FAR_SPEC>;
#[doc = "Field `FADD` reader - ECC error failing address"]
pub type FADD_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - ECC error failing address"]
    #[inline(always)]
    pub fn fadd(&self) -> FADD_R {
        FADD_R::new(self.bits)
    }
}
#[doc = "RAMECC monitor x failing address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m1far::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct M1FAR_SPEC;
impl crate::RegisterSpec for M1FAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`m1far::R`](R) reader structure"]
impl crate::Readable for M1FAR_SPEC {}
#[doc = "`reset()` method sets M1FAR to value 0"]
impl crate::Resettable for M1FAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
