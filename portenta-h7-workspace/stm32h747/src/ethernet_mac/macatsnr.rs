#[doc = "Register `MACATSNR` reader"]
pub type R = crate::R<MACATSNR_SPEC>;
#[doc = "Field `AUXTSLO` reader - AUXTSLO"]
pub type AUXTSLO_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:30 - AUXTSLO"]
    #[inline(always)]
    pub fn auxtslo(&self) -> AUXTSLO_R {
        AUXTSLO_R::new(self.bits & 0x7fff_ffff)
    }
}
#[doc = "Auxiliary timestamp nanoseconds register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macatsnr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACATSNR_SPEC;
impl crate::RegisterSpec for MACATSNR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macatsnr::R`](R) reader structure"]
impl crate::Readable for MACATSNR_SPEC {}
#[doc = "`reset()` method sets MACATSNR to value 0"]
impl crate::Resettable for MACATSNR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
