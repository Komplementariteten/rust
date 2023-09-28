#[doc = "Register `DSI_CLCR` reader"]
pub type R = crate::R<DSI_CLCR_SPEC>;
#[doc = "Register `DSI_CLCR` writer"]
pub type W = crate::W<DSI_CLCR_SPEC>;
#[doc = "Field `DPCC` reader - DPCC"]
pub type DPCC_R = crate::BitReader;
#[doc = "Field `DPCC` writer - DPCC"]
pub type DPCC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ACR` reader - ACR"]
pub type ACR_R = crate::BitReader;
#[doc = "Field `ACR` writer - ACR"]
pub type ACR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - DPCC"]
    #[inline(always)]
    pub fn dpcc(&self) -> DPCC_R {
        DPCC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ACR"]
    #[inline(always)]
    pub fn acr(&self) -> ACR_R {
        ACR_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DPCC"]
    #[inline(always)]
    #[must_use]
    pub fn dpcc(&mut self) -> DPCC_W<DSI_CLCR_SPEC, 0> {
        DPCC_W::new(self)
    }
    #[doc = "Bit 1 - ACR"]
    #[inline(always)]
    #[must_use]
    pub fn acr(&mut self) -> ACR_W<DSI_CLCR_SPEC, 1> {
        ACR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "DSI Host clock lane configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_clcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_clcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DSI_CLCR_SPEC;
impl crate::RegisterSpec for DSI_CLCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_clcr::R`](R) reader structure"]
impl crate::Readable for DSI_CLCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dsi_clcr::W`](W) writer structure"]
impl crate::Writable for DSI_CLCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DSI_CLCR to value 0"]
impl crate::Resettable for DSI_CLCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
