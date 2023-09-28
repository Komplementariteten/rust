#[doc = "Register `DSI_WCR` reader"]
pub type R = crate::R<DSI_WCR_SPEC>;
#[doc = "Register `DSI_WCR` writer"]
pub type W = crate::W<DSI_WCR_SPEC>;
#[doc = "Field `COLM` reader - COLM"]
pub type COLM_R = crate::BitReader;
#[doc = "Field `COLM` writer - COLM"]
pub type COLM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SHTDN` reader - SHTDN"]
pub type SHTDN_R = crate::BitReader;
#[doc = "Field `SHTDN` writer - SHTDN"]
pub type SHTDN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LTDCEN` reader - LTDCEN"]
pub type LTDCEN_R = crate::BitReader;
#[doc = "Field `LTDCEN` writer - LTDCEN"]
pub type LTDCEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DSIEN` reader - DSIEN"]
pub type DSIEN_R = crate::BitReader;
#[doc = "Field `DSIEN` writer - DSIEN"]
pub type DSIEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - COLM"]
    #[inline(always)]
    pub fn colm(&self) -> COLM_R {
        COLM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SHTDN"]
    #[inline(always)]
    pub fn shtdn(&self) -> SHTDN_R {
        SHTDN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - LTDCEN"]
    #[inline(always)]
    pub fn ltdcen(&self) -> LTDCEN_R {
        LTDCEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DSIEN"]
    #[inline(always)]
    pub fn dsien(&self) -> DSIEN_R {
        DSIEN_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - COLM"]
    #[inline(always)]
    #[must_use]
    pub fn colm(&mut self) -> COLM_W<DSI_WCR_SPEC, 0> {
        COLM_W::new(self)
    }
    #[doc = "Bit 1 - SHTDN"]
    #[inline(always)]
    #[must_use]
    pub fn shtdn(&mut self) -> SHTDN_W<DSI_WCR_SPEC, 1> {
        SHTDN_W::new(self)
    }
    #[doc = "Bit 2 - LTDCEN"]
    #[inline(always)]
    #[must_use]
    pub fn ltdcen(&mut self) -> LTDCEN_W<DSI_WCR_SPEC, 2> {
        LTDCEN_W::new(self)
    }
    #[doc = "Bit 3 - DSIEN"]
    #[inline(always)]
    #[must_use]
    pub fn dsien(&mut self) -> DSIEN_W<DSI_WCR_SPEC, 3> {
        DSIEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "DSI wrapper control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_wcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_wcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DSI_WCR_SPEC;
impl crate::RegisterSpec for DSI_WCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_wcr::R`](R) reader structure"]
impl crate::Readable for DSI_WCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dsi_wcr::W`](W) writer structure"]
impl crate::Writable for DSI_WCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DSI_WCR to value 0"]
impl crate::Resettable for DSI_WCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
