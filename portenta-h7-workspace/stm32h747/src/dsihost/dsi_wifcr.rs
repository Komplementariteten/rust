#[doc = "Register `DSI_WIFCR` writer"]
pub type W = crate::W<DSI_WIFCR_SPEC>;
#[doc = "Field `CTEIF` writer - CTEIF"]
pub type CTEIF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CERIF` writer - CERIF"]
pub type CERIF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CPLLLIF` writer - CPLLLIF"]
pub type CPLLLIF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CPLLUIF` writer - CPLLUIF"]
pub type CPLLUIF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CRRIF` writer - CRRIF"]
pub type CRRIF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 0 - CTEIF"]
    #[inline(always)]
    #[must_use]
    pub fn cteif(&mut self) -> CTEIF_W<DSI_WIFCR_SPEC, 0> {
        CTEIF_W::new(self)
    }
    #[doc = "Bit 1 - CERIF"]
    #[inline(always)]
    #[must_use]
    pub fn cerif(&mut self) -> CERIF_W<DSI_WIFCR_SPEC, 1> {
        CERIF_W::new(self)
    }
    #[doc = "Bit 9 - CPLLLIF"]
    #[inline(always)]
    #[must_use]
    pub fn cplllif(&mut self) -> CPLLLIF_W<DSI_WIFCR_SPEC, 9> {
        CPLLLIF_W::new(self)
    }
    #[doc = "Bit 10 - CPLLUIF"]
    #[inline(always)]
    #[must_use]
    pub fn cplluif(&mut self) -> CPLLUIF_W<DSI_WIFCR_SPEC, 10> {
        CPLLUIF_W::new(self)
    }
    #[doc = "Bit 13 - CRRIF"]
    #[inline(always)]
    #[must_use]
    pub fn crrif(&mut self) -> CRRIF_W<DSI_WIFCR_SPEC, 13> {
        CRRIF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "DSI wrapper interrupt flag clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_wifcr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DSI_WIFCR_SPEC;
impl crate::RegisterSpec for DSI_WIFCR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dsi_wifcr::W`](W) writer structure"]
impl crate::Writable for DSI_WIFCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DSI_WIFCR to value 0"]
impl crate::Resettable for DSI_WIFCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
