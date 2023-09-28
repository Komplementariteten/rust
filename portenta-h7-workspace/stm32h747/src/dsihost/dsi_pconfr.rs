#[doc = "Register `DSI_PCONFR` reader"]
pub type R = crate::R<DSI_PCONFR_SPEC>;
#[doc = "Register `DSI_PCONFR` writer"]
pub type W = crate::W<DSI_PCONFR_SPEC>;
#[doc = "Field `NL` reader - NL"]
pub type NL_R = crate::FieldReader;
#[doc = "Field `NL` writer - NL"]
pub type NL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `SW_TIME` reader - SW_TIME"]
pub type SW_TIME_R = crate::FieldReader;
#[doc = "Field `SW_TIME` writer - SW_TIME"]
pub type SW_TIME_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:1 - NL"]
    #[inline(always)]
    pub fn nl(&self) -> NL_R {
        NL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:15 - SW_TIME"]
    #[inline(always)]
    pub fn sw_time(&self) -> SW_TIME_R {
        SW_TIME_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - NL"]
    #[inline(always)]
    #[must_use]
    pub fn nl(&mut self) -> NL_W<DSI_PCONFR_SPEC, 0> {
        NL_W::new(self)
    }
    #[doc = "Bits 8:15 - SW_TIME"]
    #[inline(always)]
    #[must_use]
    pub fn sw_time(&mut self) -> SW_TIME_W<DSI_PCONFR_SPEC, 8> {
        SW_TIME_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "DSI Host PHY configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_pconfr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_pconfr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DSI_PCONFR_SPEC;
impl crate::RegisterSpec for DSI_PCONFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_pconfr::R`](R) reader structure"]
impl crate::Readable for DSI_PCONFR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dsi_pconfr::W`](W) writer structure"]
impl crate::Writable for DSI_PCONFR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DSI_PCONFR to value 0x01"]
impl crate::Resettable for DSI_PCONFR_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
