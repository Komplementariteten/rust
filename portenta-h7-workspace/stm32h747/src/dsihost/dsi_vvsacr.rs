#[doc = "Register `DSI_VVSACR` reader"]
pub type R = crate::R<DSI_VVSACR_SPEC>;
#[doc = "Register `DSI_VVSACR` writer"]
pub type W = crate::W<DSI_VVSACR_SPEC>;
#[doc = "Field `VSA` reader - VSA"]
pub type VSA_R = crate::FieldReader<u16>;
#[doc = "Field `VSA` writer - VSA"]
pub type VSA_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 10, O, u16>;
impl R {
    #[doc = "Bits 0:9 - VSA"]
    #[inline(always)]
    pub fn vsa(&self) -> VSA_R {
        VSA_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - VSA"]
    #[inline(always)]
    #[must_use]
    pub fn vsa(&mut self) -> VSA_W<DSI_VVSACR_SPEC, 0> {
        VSA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "DSI Host video VSA configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_vvsacr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_vvsacr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DSI_VVSACR_SPEC;
impl crate::RegisterSpec for DSI_VVSACR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_vvsacr::R`](R) reader structure"]
impl crate::Readable for DSI_VVSACR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dsi_vvsacr::W`](W) writer structure"]
impl crate::Writable for DSI_VVSACR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DSI_VVSACR to value 0"]
impl crate::Resettable for DSI_VVSACR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
