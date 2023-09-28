#[doc = "Register `VERR` reader"]
pub type R = crate::R<VERR_SPEC>;
#[doc = "Field `MINREV` reader - Minor revision"]
pub type MINREV_R = crate::FieldReader;
#[doc = "Field `MAJREV` reader - Major revision"]
pub type MAJREV_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Minor revision"]
    #[inline(always)]
    pub fn minrev(&self) -> MINREV_R {
        MINREV_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Major revision"]
    #[inline(always)]
    pub fn majrev(&self) -> MAJREV_R {
        MAJREV_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "SPDIFRX version register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`verr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VERR_SPEC;
impl crate::RegisterSpec for VERR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`verr::R`](R) reader structure"]
impl crate::Readable for VERR_SPEC {}
#[doc = "`reset()` method sets VERR to value 0x12"]
impl crate::Resettable for VERR_SPEC {
    const RESET_VALUE: Self::Ux = 0x12;
}
