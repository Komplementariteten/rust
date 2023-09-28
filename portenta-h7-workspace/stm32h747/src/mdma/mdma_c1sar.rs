#[doc = "Register `MDMA_C1SAR` reader"]
pub type R = crate::R<MDMA_C1SAR_SPEC>;
#[doc = "Register `MDMA_C1SAR` writer"]
pub type W = crate::W<MDMA_C1SAR_SPEC>;
#[doc = "Field `SAR` reader - source adr base"]
pub type SAR_R = crate::FieldReader<u32>;
#[doc = "Field `SAR` writer - source adr base"]
pub type SAR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - source adr base"]
    #[inline(always)]
    pub fn sar(&self) -> SAR_R {
        SAR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - source adr base"]
    #[inline(always)]
    #[must_use]
    pub fn sar(&mut self) -> SAR_W<MDMA_C1SAR_SPEC, 0> {
        SAR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "MDMA channel x source address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c1sar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c1sar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MDMA_C1SAR_SPEC;
impl crate::RegisterSpec for MDMA_C1SAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdma_c1sar::R`](R) reader structure"]
impl crate::Readable for MDMA_C1SAR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mdma_c1sar::W`](W) writer structure"]
impl crate::Writable for MDMA_C1SAR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MDMA_C1SAR to value 0"]
impl crate::Resettable for MDMA_C1SAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
