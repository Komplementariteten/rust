#[doc = "Register `MTLISR` reader"]
pub type R = crate::R<MTLISR_SPEC>;
#[doc = "Field `Q0IS` reader - Queue interrupt status"]
pub type Q0IS_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Queue interrupt status"]
    #[inline(always)]
    pub fn q0is(&self) -> Q0IS_R {
        Q0IS_R::new((self.bits & 1) != 0)
    }
}
#[doc = "Interrupt status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mtlisr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MTLISR_SPEC;
impl crate::RegisterSpec for MTLISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mtlisr::R`](R) reader structure"]
impl crate::Readable for MTLISR_SPEC {}
#[doc = "`reset()` method sets MTLISR to value 0"]
impl crate::Resettable for MTLISR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
