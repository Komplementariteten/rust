#[doc = "Register `CPT2ER` reader"]
pub type R = crate::R<CPT2ER_SPEC>;
#[doc = "Field `CPT2x` reader - Timerx Capture 2 value"]
pub type CPT2X_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Timerx Capture 2 value"]
    #[inline(always)]
    pub fn cpt2x(&self) -> CPT2X_R {
        CPT2X_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Timerx Capture 2 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpt2er::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CPT2ER_SPEC;
impl crate::RegisterSpec for CPT2ER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpt2er::R`](R) reader structure"]
impl crate::Readable for CPT2ER_SPEC {}
#[doc = "`reset()` method sets CPT2ER to value 0"]
impl crate::Resettable for CPT2ER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
