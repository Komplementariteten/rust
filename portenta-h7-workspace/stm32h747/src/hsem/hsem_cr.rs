#[doc = "Register `HSEM_CR` writer"]
pub type W = crate::W<HSEM_CR_SPEC>;
#[doc = "Field `COREID` writer - COREID of semaphores to be cleared"]
pub type COREID_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `KEY` writer - Semaphore clear Key"]
pub type KEY_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl W {
    #[doc = "Bits 8:11 - COREID of semaphores to be cleared"]
    #[inline(always)]
    #[must_use]
    pub fn coreid(&mut self) -> COREID_W<HSEM_CR_SPEC, 8> {
        COREID_W::new(self)
    }
    #[doc = "Bits 16:31 - Semaphore clear Key"]
    #[inline(always)]
    #[must_use]
    pub fn key(&mut self) -> KEY_W<HSEM_CR_SPEC, 16> {
        KEY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "HSEM Clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hsem_cr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HSEM_CR_SPEC;
impl crate::RegisterSpec for HSEM_CR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`hsem_cr::W`](W) writer structure"]
impl crate::Writable for HSEM_CR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HSEM_CR to value 0"]
impl crate::Resettable for HSEM_CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
