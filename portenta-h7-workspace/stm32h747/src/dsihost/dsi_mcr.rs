#[doc = "Register `DSI_MCR` reader"]
pub type R = crate::R<DSI_MCR_SPEC>;
#[doc = "Register `DSI_MCR` writer"]
pub type W = crate::W<DSI_MCR_SPEC>;
#[doc = "Field `CMDM` reader - CMDM"]
pub type CMDM_R = crate::BitReader;
#[doc = "Field `CMDM` writer - CMDM"]
pub type CMDM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - CMDM"]
    #[inline(always)]
    pub fn cmdm(&self) -> CMDM_R {
        CMDM_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CMDM"]
    #[inline(always)]
    #[must_use]
    pub fn cmdm(&mut self) -> CMDM_W<DSI_MCR_SPEC, 0> {
        CMDM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "DSI Host mode configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_mcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_mcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DSI_MCR_SPEC;
impl crate::RegisterSpec for DSI_MCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_mcr::R`](R) reader structure"]
impl crate::Readable for DSI_MCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dsi_mcr::W`](W) writer structure"]
impl crate::Writable for DSI_MCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DSI_MCR to value 0x01"]
impl crate::Resettable for DSI_MCR_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
