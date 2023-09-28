#[doc = "Register `DSI_VCCR` reader"]
pub type R = crate::R<DSI_VCCR_SPEC>;
#[doc = "Register `DSI_VCCR` writer"]
pub type W = crate::W<DSI_VCCR_SPEC>;
#[doc = "Field `NUMC` reader - NUMC"]
pub type NUMC_R = crate::FieldReader<u16>;
#[doc = "Field `NUMC` writer - NUMC"]
pub type NUMC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 13, O, u16>;
impl R {
    #[doc = "Bits 0:12 - NUMC"]
    #[inline(always)]
    pub fn numc(&self) -> NUMC_R {
        NUMC_R::new((self.bits & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - NUMC"]
    #[inline(always)]
    #[must_use]
    pub fn numc(&mut self) -> NUMC_W<DSI_VCCR_SPEC, 0> {
        NUMC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "DSI Host video chunks configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_vccr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_vccr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DSI_VCCR_SPEC;
impl crate::RegisterSpec for DSI_VCCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_vccr::R`](R) reader structure"]
impl crate::Readable for DSI_VCCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dsi_vccr::W`](W) writer structure"]
impl crate::Writable for DSI_VCCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DSI_VCCR to value 0"]
impl crate::Resettable for DSI_VCCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
