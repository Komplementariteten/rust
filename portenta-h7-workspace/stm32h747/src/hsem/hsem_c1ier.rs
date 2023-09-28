#[doc = "Register `HSEM_C1IER` reader"]
pub type R = crate::R<HSEM_C1IER_SPEC>;
#[doc = "Register `HSEM_C1IER` writer"]
pub type W = crate::W<HSEM_C1IER_SPEC>;
#[doc = "Field `ISE` reader - Interrupt semaphore x enable bit"]
pub type ISE_R = crate::FieldReader<u32>;
#[doc = "Field `ISE` writer - Interrupt semaphore x enable bit"]
pub type ISE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Interrupt semaphore x enable bit"]
    #[inline(always)]
    pub fn ise(&self) -> ISE_R {
        ISE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Interrupt semaphore x enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn ise(&mut self) -> ISE_W<HSEM_C1IER_SPEC, 0> {
        ISE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "HSEM Interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hsem_c1ier::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hsem_c1ier::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HSEM_C1IER_SPEC;
impl crate::RegisterSpec for HSEM_C1IER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hsem_c1ier::R`](R) reader structure"]
impl crate::Readable for HSEM_C1IER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hsem_c1ier::W`](W) writer structure"]
impl crate::Writable for HSEM_C1IER_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HSEM_C1IER to value 0"]
impl crate::Resettable for HSEM_C1IER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
