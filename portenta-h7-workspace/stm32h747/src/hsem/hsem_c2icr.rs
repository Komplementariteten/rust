#[doc = "Register `HSEM_C2ICR` reader"]
pub type R = crate::R<HSEM_C2ICR_SPEC>;
#[doc = "Register `HSEM_C2ICR` writer"]
pub type W = crate::W<HSEM_C2ICR_SPEC>;
#[doc = "Field `ISC` reader - Interrupt semaphore x clear bit"]
pub type ISC_R = crate::FieldReader<u32>;
#[doc = "Field `ISC` writer - Interrupt semaphore x clear bit"]
pub type ISC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Interrupt semaphore x clear bit"]
    #[inline(always)]
    pub fn isc(&self) -> ISC_R {
        ISC_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Interrupt semaphore x clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn isc(&mut self) -> ISC_W<HSEM_C2ICR_SPEC, 0> {
        ISC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "HSEM Interrupt clear register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hsem_c2icr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hsem_c2icr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HSEM_C2ICR_SPEC;
impl crate::RegisterSpec for HSEM_C2ICR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hsem_c2icr::R`](R) reader structure"]
impl crate::Readable for HSEM_C2ICR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hsem_c2icr::W`](W) writer structure"]
impl crate::Writable for HSEM_C2ICR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HSEM_C2ICR to value 0"]
impl crate::Resettable for HSEM_C2ICR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
