#[doc = "Register `MDMA_C8MAR` reader"]
pub type R = crate::R<MDMA_C8MAR_SPEC>;
#[doc = "Register `MDMA_C8MAR` writer"]
pub type W = crate::W<MDMA_C8MAR_SPEC>;
#[doc = "Field `MAR` reader - Mask address"]
pub type MAR_R = crate::FieldReader<u32>;
#[doc = "Field `MAR` writer - Mask address"]
pub type MAR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Mask address"]
    #[inline(always)]
    pub fn mar(&self) -> MAR_R {
        MAR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Mask address"]
    #[inline(always)]
    #[must_use]
    pub fn mar(&mut self) -> MAR_W<MDMA_C8MAR_SPEC, 0> {
        MAR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "MDMA channel x Mask address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c8mar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c8mar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MDMA_C8MAR_SPEC;
impl crate::RegisterSpec for MDMA_C8MAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdma_c8mar::R`](R) reader structure"]
impl crate::Readable for MDMA_C8MAR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mdma_c8mar::W`](W) writer structure"]
impl crate::Writable for MDMA_C8MAR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MDMA_C8MAR to value 0"]
impl crate::Resettable for MDMA_C8MAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
