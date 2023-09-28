#[doc = "Register `DSI_PCTLR` reader"]
pub type R = crate::R<DSI_PCTLR_SPEC>;
#[doc = "Register `DSI_PCTLR` writer"]
pub type W = crate::W<DSI_PCTLR_SPEC>;
#[doc = "Field `DEN` reader - DEN"]
pub type DEN_R = crate::BitReader;
#[doc = "Field `DEN` writer - DEN"]
pub type DEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CKE` reader - CKE"]
pub type CKE_R = crate::BitReader;
#[doc = "Field `CKE` writer - CKE"]
pub type CKE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 1 - DEN"]
    #[inline(always)]
    pub fn den(&self) -> DEN_R {
        DEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CKE"]
    #[inline(always)]
    pub fn cke(&self) -> CKE_R {
        CKE_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - DEN"]
    #[inline(always)]
    #[must_use]
    pub fn den(&mut self) -> DEN_W<DSI_PCTLR_SPEC, 1> {
        DEN_W::new(self)
    }
    #[doc = "Bit 2 - CKE"]
    #[inline(always)]
    #[must_use]
    pub fn cke(&mut self) -> CKE_W<DSI_PCTLR_SPEC, 2> {
        CKE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "DSI Host PHY control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_pctlr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_pctlr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DSI_PCTLR_SPEC;
impl crate::RegisterSpec for DSI_PCTLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_pctlr::R`](R) reader structure"]
impl crate::Readable for DSI_PCTLR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dsi_pctlr::W`](W) writer structure"]
impl crate::Writable for DSI_PCTLR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DSI_PCTLR to value 0"]
impl crate::Resettable for DSI_PCTLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
