#[doc = "Register `DSI_CLTCR` reader"]
pub type R = crate::R<DSI_CLTCR_SPEC>;
#[doc = "Register `DSI_CLTCR` writer"]
pub type W = crate::W<DSI_CLTCR_SPEC>;
#[doc = "Field `LP2HS_TIME` reader - LP2HS_TIME"]
pub type LP2HS_TIME_R = crate::FieldReader<u16>;
#[doc = "Field `LP2HS_TIME` writer - LP2HS_TIME"]
pub type LP2HS_TIME_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 10, O, u16>;
#[doc = "Field `HS2LP_TIME` reader - HS2LP_TIME"]
pub type HS2LP_TIME_R = crate::FieldReader<u16>;
#[doc = "Field `HS2LP_TIME` writer - HS2LP_TIME"]
pub type HS2LP_TIME_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 10, O, u16>;
impl R {
    #[doc = "Bits 0:9 - LP2HS_TIME"]
    #[inline(always)]
    pub fn lp2hs_time(&self) -> LP2HS_TIME_R {
        LP2HS_TIME_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - HS2LP_TIME"]
    #[inline(always)]
    pub fn hs2lp_time(&self) -> HS2LP_TIME_R {
        HS2LP_TIME_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - LP2HS_TIME"]
    #[inline(always)]
    #[must_use]
    pub fn lp2hs_time(&mut self) -> LP2HS_TIME_W<DSI_CLTCR_SPEC, 0> {
        LP2HS_TIME_W::new(self)
    }
    #[doc = "Bits 16:25 - HS2LP_TIME"]
    #[inline(always)]
    #[must_use]
    pub fn hs2lp_time(&mut self) -> HS2LP_TIME_W<DSI_CLTCR_SPEC, 16> {
        HS2LP_TIME_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "DSI Host clock lane timer configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_cltcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_cltcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DSI_CLTCR_SPEC;
impl crate::RegisterSpec for DSI_CLTCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_cltcr::R`](R) reader structure"]
impl crate::Readable for DSI_CLTCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dsi_cltcr::W`](W) writer structure"]
impl crate::Writable for DSI_CLTCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DSI_CLTCR to value 0"]
impl crate::Resettable for DSI_CLTCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
