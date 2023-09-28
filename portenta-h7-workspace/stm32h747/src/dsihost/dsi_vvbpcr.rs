#[doc = "Register `DSI_VVBPCR` reader"]
pub type R = crate::R<DSI_VVBPCR_SPEC>;
#[doc = "Register `DSI_VVBPCR` writer"]
pub type W = crate::W<DSI_VVBPCR_SPEC>;
#[doc = "Field `VBP` reader - VBP"]
pub type VBP_R = crate::FieldReader<u16>;
#[doc = "Field `VBP` writer - VBP"]
pub type VBP_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 10, O, u16>;
impl R {
    #[doc = "Bits 0:9 - VBP"]
    #[inline(always)]
    pub fn vbp(&self) -> VBP_R {
        VBP_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - VBP"]
    #[inline(always)]
    #[must_use]
    pub fn vbp(&mut self) -> VBP_W<DSI_VVBPCR_SPEC, 0> {
        VBP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "DSI Host video VBP configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_vvbpcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_vvbpcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DSI_VVBPCR_SPEC;
impl crate::RegisterSpec for DSI_VVBPCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_vvbpcr::R`](R) reader structure"]
impl crate::Readable for DSI_VVBPCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dsi_vvbpcr::W`](W) writer structure"]
impl crate::Writable for DSI_VVBPCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DSI_VVBPCR to value 0"]
impl crate::Resettable for DSI_VVBPCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
