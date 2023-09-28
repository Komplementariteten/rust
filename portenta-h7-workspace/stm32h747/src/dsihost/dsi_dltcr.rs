#[doc = "Register `DSI_DLTCR` reader"]
pub type R = crate::R<DSI_DLTCR_SPEC>;
#[doc = "Register `DSI_DLTCR` writer"]
pub type W = crate::W<DSI_DLTCR_SPEC>;
#[doc = "Field `MRD_TIME` reader - Maximum read time"]
pub type MRD_TIME_R = crate::FieldReader<u16>;
#[doc = "Field `MRD_TIME` writer - Maximum read time"]
pub type MRD_TIME_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 15, O, u16>;
#[doc = "Field `LP2HS_TIME` reader - LP2HS_TIME"]
pub type LP2HS_TIME_R = crate::FieldReader;
#[doc = "Field `LP2HS_TIME` writer - LP2HS_TIME"]
pub type LP2HS_TIME_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `HS2LP_TIME` reader - HS2LP_TIME"]
pub type HS2LP_TIME_R = crate::FieldReader;
#[doc = "Field `HS2LP_TIME` writer - HS2LP_TIME"]
pub type HS2LP_TIME_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:14 - Maximum read time"]
    #[inline(always)]
    pub fn mrd_time(&self) -> MRD_TIME_R {
        MRD_TIME_R::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bits 16:23 - LP2HS_TIME"]
    #[inline(always)]
    pub fn lp2hs_time(&self) -> LP2HS_TIME_R {
        LP2HS_TIME_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - HS2LP_TIME"]
    #[inline(always)]
    pub fn hs2lp_time(&self) -> HS2LP_TIME_R {
        HS2LP_TIME_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:14 - Maximum read time"]
    #[inline(always)]
    #[must_use]
    pub fn mrd_time(&mut self) -> MRD_TIME_W<DSI_DLTCR_SPEC, 0> {
        MRD_TIME_W::new(self)
    }
    #[doc = "Bits 16:23 - LP2HS_TIME"]
    #[inline(always)]
    #[must_use]
    pub fn lp2hs_time(&mut self) -> LP2HS_TIME_W<DSI_DLTCR_SPEC, 16> {
        LP2HS_TIME_W::new(self)
    }
    #[doc = "Bits 24:31 - HS2LP_TIME"]
    #[inline(always)]
    #[must_use]
    pub fn hs2lp_time(&mut self) -> HS2LP_TIME_W<DSI_DLTCR_SPEC, 24> {
        HS2LP_TIME_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "DSI Host data lane timer configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_dltcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_dltcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DSI_DLTCR_SPEC;
impl crate::RegisterSpec for DSI_DLTCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_dltcr::R`](R) reader structure"]
impl crate::Readable for DSI_DLTCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dsi_dltcr::W`](W) writer structure"]
impl crate::Writable for DSI_DLTCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DSI_DLTCR to value 0"]
impl crate::Resettable for DSI_DLTCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
