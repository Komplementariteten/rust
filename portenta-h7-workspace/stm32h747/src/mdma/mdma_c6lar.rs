#[doc = "Register `MDMA_C6LAR` reader"]
pub type R = crate::R<MDMA_C6LAR_SPEC>;
#[doc = "Register `MDMA_C6LAR` writer"]
pub type W = crate::W<MDMA_C6LAR_SPEC>;
#[doc = "Field `LAR` reader - Link address register"]
pub type LAR_R = crate::FieldReader<u32>;
#[doc = "Field `LAR` writer - Link address register"]
pub type LAR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Link address register"]
    #[inline(always)]
    pub fn lar(&self) -> LAR_R {
        LAR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Link address register"]
    #[inline(always)]
    #[must_use]
    pub fn lar(&mut self) -> LAR_W<MDMA_C6LAR_SPEC, 0> {
        LAR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "MDMA channel x Link Address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c6lar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c6lar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MDMA_C6LAR_SPEC;
impl crate::RegisterSpec for MDMA_C6LAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdma_c6lar::R`](R) reader structure"]
impl crate::Readable for MDMA_C6LAR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mdma_c6lar::W`](W) writer structure"]
impl crate::Writable for MDMA_C6LAR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MDMA_C6LAR to value 0"]
impl crate::Resettable for MDMA_C6LAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
