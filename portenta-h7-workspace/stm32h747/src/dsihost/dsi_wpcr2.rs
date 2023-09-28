#[doc = "Register `DSI_WPCR2` reader"]
pub type R = crate::R<DSI_WPCR2_SPEC>;
#[doc = "Register `DSI_WPCR2` writer"]
pub type W = crate::W<DSI_WPCR2_SPEC>;
#[doc = "Field `TCLKPREP` reader - TCLKPREP"]
pub type TCLKPREP_R = crate::FieldReader;
#[doc = "Field `TCLKPREP` writer - TCLKPREP"]
pub type TCLKPREP_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `TCLKZERO` reader - TCLKZERO"]
pub type TCLKZERO_R = crate::FieldReader;
#[doc = "Field `TCLKZERO` writer - TCLKZERO"]
pub type TCLKZERO_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `THSPREP` reader - THSPREP"]
pub type THSPREP_R = crate::FieldReader;
#[doc = "Field `THSPREP` writer - THSPREP"]
pub type THSPREP_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `THSTRAIL` reader - THSTRAIL"]
pub type THSTRAIL_R = crate::FieldReader;
#[doc = "Field `THSTRAIL` writer - THSTRAIL"]
pub type THSTRAIL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - TCLKPREP"]
    #[inline(always)]
    pub fn tclkprep(&self) -> TCLKPREP_R {
        TCLKPREP_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - TCLKZERO"]
    #[inline(always)]
    pub fn tclkzero(&self) -> TCLKZERO_R {
        TCLKZERO_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - THSPREP"]
    #[inline(always)]
    pub fn thsprep(&self) -> THSPREP_R {
        THSPREP_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - THSTRAIL"]
    #[inline(always)]
    pub fn thstrail(&self) -> THSTRAIL_R {
        THSTRAIL_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - TCLKPREP"]
    #[inline(always)]
    #[must_use]
    pub fn tclkprep(&mut self) -> TCLKPREP_W<DSI_WPCR2_SPEC, 0> {
        TCLKPREP_W::new(self)
    }
    #[doc = "Bits 8:15 - TCLKZERO"]
    #[inline(always)]
    #[must_use]
    pub fn tclkzero(&mut self) -> TCLKZERO_W<DSI_WPCR2_SPEC, 8> {
        TCLKZERO_W::new(self)
    }
    #[doc = "Bits 16:23 - THSPREP"]
    #[inline(always)]
    #[must_use]
    pub fn thsprep(&mut self) -> THSPREP_W<DSI_WPCR2_SPEC, 16> {
        THSPREP_W::new(self)
    }
    #[doc = "Bits 24:31 - THSTRAIL"]
    #[inline(always)]
    #[must_use]
    pub fn thstrail(&mut self) -> THSTRAIL_W<DSI_WPCR2_SPEC, 24> {
        THSTRAIL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "DSI wrapper PHY configuration register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_wpcr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_wpcr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DSI_WPCR2_SPEC;
impl crate::RegisterSpec for DSI_WPCR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_wpcr2::R`](R) reader structure"]
impl crate::Readable for DSI_WPCR2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dsi_wpcr2::W`](W) writer structure"]
impl crate::Writable for DSI_WPCR2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DSI_WPCR2 to value 0"]
impl crate::Resettable for DSI_WPCR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
