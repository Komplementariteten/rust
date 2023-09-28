#[doc = "Register `DSI_VLCR` reader"]
pub type R = crate::R<DSI_VLCR_SPEC>;
#[doc = "Register `DSI_VLCR` writer"]
pub type W = crate::W<DSI_VLCR_SPEC>;
#[doc = "Field `HLINE` reader - HLINE"]
pub type HLINE_R = crate::FieldReader<u16>;
#[doc = "Field `HLINE` writer - HLINE"]
pub type HLINE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 15, O, u16>;
impl R {
    #[doc = "Bits 0:14 - HLINE"]
    #[inline(always)]
    pub fn hline(&self) -> HLINE_R {
        HLINE_R::new((self.bits & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:14 - HLINE"]
    #[inline(always)]
    #[must_use]
    pub fn hline(&mut self) -> HLINE_W<DSI_VLCR_SPEC, 0> {
        HLINE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "DSI Host video line configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_vlcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_vlcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DSI_VLCR_SPEC;
impl crate::RegisterSpec for DSI_VLCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_vlcr::R`](R) reader structure"]
impl crate::Readable for DSI_VLCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dsi_vlcr::W`](W) writer structure"]
impl crate::Writable for DSI_VLCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DSI_VLCR to value 0"]
impl crate::Resettable for DSI_VLCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
