#[doc = "Register `DSI_LPCR` reader"]
pub type R = crate::R<DSI_LPCR_SPEC>;
#[doc = "Register `DSI_LPCR` writer"]
pub type W = crate::W<DSI_LPCR_SPEC>;
#[doc = "Field `DEP` reader - DEP"]
pub type DEP_R = crate::BitReader;
#[doc = "Field `DEP` writer - DEP"]
pub type DEP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `VSP` reader - VSP"]
pub type VSP_R = crate::BitReader;
#[doc = "Field `VSP` writer - VSP"]
pub type VSP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HSP` reader - HSP"]
pub type HSP_R = crate::BitReader;
#[doc = "Field `HSP` writer - HSP"]
pub type HSP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - DEP"]
    #[inline(always)]
    pub fn dep(&self) -> DEP_R {
        DEP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - VSP"]
    #[inline(always)]
    pub fn vsp(&self) -> VSP_R {
        VSP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - HSP"]
    #[inline(always)]
    pub fn hsp(&self) -> HSP_R {
        HSP_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DEP"]
    #[inline(always)]
    #[must_use]
    pub fn dep(&mut self) -> DEP_W<DSI_LPCR_SPEC, 0> {
        DEP_W::new(self)
    }
    #[doc = "Bit 1 - VSP"]
    #[inline(always)]
    #[must_use]
    pub fn vsp(&mut self) -> VSP_W<DSI_LPCR_SPEC, 1> {
        VSP_W::new(self)
    }
    #[doc = "Bit 2 - HSP"]
    #[inline(always)]
    #[must_use]
    pub fn hsp(&mut self) -> HSP_W<DSI_LPCR_SPEC, 2> {
        HSP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "DSI Host LTDC polarity configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_lpcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_lpcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DSI_LPCR_SPEC;
impl crate::RegisterSpec for DSI_LPCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_lpcr::R`](R) reader structure"]
impl crate::Readable for DSI_LPCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dsi_lpcr::W`](W) writer structure"]
impl crate::Writable for DSI_LPCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DSI_LPCR to value 0"]
impl crate::Resettable for DSI_LPCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
