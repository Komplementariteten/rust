#[doc = "Register `CRRCR` reader"]
pub type R = crate::R<CRRCR_SPEC>;
#[doc = "Field `RC48CAL` reader - Internal RC 48 MHz clock calibration"]
pub type RC48CAL_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:9 - Internal RC 48 MHz clock calibration"]
    #[inline(always)]
    pub fn rc48cal(&self) -> RC48CAL_R {
        RC48CAL_R::new((self.bits & 0x03ff) as u16)
    }
}
#[doc = "RCC Clock Recovery RC Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crrcr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CRRCR_SPEC;
impl crate::RegisterSpec for CRRCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crrcr::R`](R) reader structure"]
impl crate::Readable for CRRCR_SPEC {}
#[doc = "`reset()` method sets CRRCR to value 0"]
impl crate::Resettable for CRRCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
