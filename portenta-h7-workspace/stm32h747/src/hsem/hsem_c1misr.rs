#[doc = "Register `HSEM_C1MISR` reader"]
pub type R = crate::R<HSEM_C1MISR_SPEC>;
#[doc = "Field `MISF` reader - masked interrupt semaphore x status bit after enable (mask)"]
pub type MISF_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - masked interrupt semaphore x status bit after enable (mask)"]
    #[inline(always)]
    pub fn misf(&self) -> MISF_R {
        MISF_R::new(self.bits)
    }
}
#[doc = "HSEM Masked interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hsem_c1misr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HSEM_C1MISR_SPEC;
impl crate::RegisterSpec for HSEM_C1MISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hsem_c1misr::R`](R) reader structure"]
impl crate::Readable for HSEM_C1MISR_SPEC {}
#[doc = "`reset()` method sets HSEM_C1MISR to value 0"]
impl crate::Resettable for HSEM_C1MISR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
