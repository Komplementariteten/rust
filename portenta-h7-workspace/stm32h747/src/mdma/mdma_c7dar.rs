#[doc = "Register `MDMA_C7DAR` reader"]
pub type R = crate::R<MDMA_C7DAR_SPEC>;
#[doc = "Register `MDMA_C7DAR` writer"]
pub type W = crate::W<MDMA_C7DAR_SPEC>;
#[doc = "Field `DAR` reader - Destination adr base"]
pub type DAR_R = crate::FieldReader<u32>;
#[doc = "Field `DAR` writer - Destination adr base"]
pub type DAR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Destination adr base"]
    #[inline(always)]
    pub fn dar(&self) -> DAR_R {
        DAR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Destination adr base"]
    #[inline(always)]
    #[must_use]
    pub fn dar(&mut self) -> DAR_W<MDMA_C7DAR_SPEC, 0> {
        DAR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "MDMA channel x destination address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c7dar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c7dar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MDMA_C7DAR_SPEC;
impl crate::RegisterSpec for MDMA_C7DAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdma_c7dar::R`](R) reader structure"]
impl crate::Readable for MDMA_C7DAR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mdma_c7dar::W`](W) writer structure"]
impl crate::Writable for MDMA_C7DAR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MDMA_C7DAR to value 0"]
impl crate::Resettable for MDMA_C7DAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
