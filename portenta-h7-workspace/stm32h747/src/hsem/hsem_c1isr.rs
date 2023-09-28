#[doc = "Register `HSEM_C1ISR` reader"]
pub type R = crate::R<HSEM_C1ISR_SPEC>;
#[doc = "Field `ISF` reader - Interrupt semaphore x status bit before enable (mask)"]
pub type ISF_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Interrupt semaphore x status bit before enable (mask)"]
    #[inline(always)]
    pub fn isf(&self) -> ISF_R {
        ISF_R::new(self.bits)
    }
}
#[doc = "HSEM Interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hsem_c1isr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HSEM_C1ISR_SPEC;
impl crate::RegisterSpec for HSEM_C1ISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hsem_c1isr::R`](R) reader structure"]
impl crate::Readable for HSEM_C1ISR_SPEC {}
#[doc = "`reset()` method sets HSEM_C1ISR to value 0"]
impl crate::Resettable for HSEM_C1ISR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
