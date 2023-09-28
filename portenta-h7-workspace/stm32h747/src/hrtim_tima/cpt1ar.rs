#[doc = "Register `CPT1AR` reader"]
pub type R = crate::R<CPT1AR_SPEC>;
#[doc = "Field `CPT1x` reader - Timerx Capture 1 value"]
pub type CPT1X_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Timerx Capture 1 value"]
    #[inline(always)]
    pub fn cpt1x(&self) -> CPT1X_R {
        CPT1X_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Timerx Capture 1 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpt1ar::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CPT1AR_SPEC;
impl crate::RegisterSpec for CPT1AR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpt1ar::R`](R) reader structure"]
impl crate::Readable for CPT1AR_SPEC {}
#[doc = "`reset()` method sets CPT1AR to value 0"]
impl crate::Resettable for CPT1AR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
