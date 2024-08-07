#[doc = "Register `M4FECR` reader"]
pub type R = crate::R<M4FECR_SPEC>;
#[doc = "Field `FDATAH` reader - Failing data high (64-bit memory)"]
pub type FDATAH_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Failing data high (64-bit memory)"]
    #[inline(always)]
    pub fn fdatah(&self) -> FDATAH_R {
        FDATAH_R::new(self.bits)
    }
}
#[doc = "RAMECC monitor x failing ECC error code register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m4fecr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct M4FECR_SPEC;
impl crate::RegisterSpec for M4FECR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`m4fecr::R`](R) reader structure"]
impl crate::Readable for M4FECR_SPEC {}
#[doc = "`reset()` method sets M4FECR to value 0"]
impl crate::Resettable for M4FECR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
