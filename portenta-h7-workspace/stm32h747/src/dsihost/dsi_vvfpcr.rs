#[doc = "Register `DSI_VVFPCR` reader"]
pub type R = crate::R<DSI_VVFPCR_SPEC>;
#[doc = "Register `DSI_VVFPCR` writer"]
pub type W = crate::W<DSI_VVFPCR_SPEC>;
#[doc = "Field `VFP` reader - VFP"]
pub type VFP_R = crate::FieldReader<u16>;
#[doc = "Field `VFP` writer - VFP"]
pub type VFP_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 10, O, u16>;
impl R {
    #[doc = "Bits 0:9 - VFP"]
    #[inline(always)]
    pub fn vfp(&self) -> VFP_R {
        VFP_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - VFP"]
    #[inline(always)]
    #[must_use]
    pub fn vfp(&mut self) -> VFP_W<DSI_VVFPCR_SPEC, 0> {
        VFP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "DSI Host video VFP configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_vvfpcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_vvfpcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DSI_VVFPCR_SPEC;
impl crate::RegisterSpec for DSI_VVFPCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_vvfpcr::R`](R) reader structure"]
impl crate::Readable for DSI_VVFPCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dsi_vvfpcr::W`](W) writer structure"]
impl crate::Writable for DSI_VVFPCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DSI_VVFPCR to value 0"]
impl crate::Resettable for DSI_VVFPCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
