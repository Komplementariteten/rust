#[doc = "Register `DSI_PTTCR` reader"]
pub type R = crate::R<DSI_PTTCR_SPEC>;
#[doc = "Register `DSI_PTTCR` writer"]
pub type W = crate::W<DSI_PTTCR_SPEC>;
#[doc = "Field `TX_TRIG` reader - TX_TRIG"]
pub type TX_TRIG_R = crate::FieldReader;
#[doc = "Field `TX_TRIG` writer - TX_TRIG"]
pub type TX_TRIG_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
impl R {
    #[doc = "Bits 0:3 - TX_TRIG"]
    #[inline(always)]
    pub fn tx_trig(&self) -> TX_TRIG_R {
        TX_TRIG_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - TX_TRIG"]
    #[inline(always)]
    #[must_use]
    pub fn tx_trig(&mut self) -> TX_TRIG_W<DSI_PTTCR_SPEC, 0> {
        TX_TRIG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "DSI Host PHY TX triggers configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_pttcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_pttcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DSI_PTTCR_SPEC;
impl crate::RegisterSpec for DSI_PTTCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_pttcr::R`](R) reader structure"]
impl crate::Readable for DSI_PTTCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dsi_pttcr::W`](W) writer structure"]
impl crate::Writable for DSI_PTTCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DSI_PTTCR to value 0"]
impl crate::Resettable for DSI_PTTCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
