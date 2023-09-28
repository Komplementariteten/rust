#[doc = "Register `DSI_WCFGR` reader"]
pub type R = crate::R<DSI_WCFGR_SPEC>;
#[doc = "Register `DSI_WCFGR` writer"]
pub type W = crate::W<DSI_WCFGR_SPEC>;
#[doc = "Field `DSIM` reader - DSIM"]
pub type DSIM_R = crate::BitReader;
#[doc = "Field `DSIM` writer - DSIM"]
pub type DSIM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `COLMUX` reader - COLMUX"]
pub type COLMUX_R = crate::FieldReader;
#[doc = "Field `COLMUX` writer - COLMUX"]
pub type COLMUX_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `TESRC` reader - TESRC"]
pub type TESRC_R = crate::BitReader;
#[doc = "Field `TESRC` writer - TESRC"]
pub type TESRC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TEPOL` reader - TEPOL"]
pub type TEPOL_R = crate::BitReader;
#[doc = "Field `TEPOL` writer - TEPOL"]
pub type TEPOL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `AR` reader - AR"]
pub type AR_R = crate::BitReader;
#[doc = "Field `AR` writer - AR"]
pub type AR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `VSPOL` reader - VSPOL"]
pub type VSPOL_R = crate::BitReader;
#[doc = "Field `VSPOL` writer - VSPOL"]
pub type VSPOL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - DSIM"]
    #[inline(always)]
    pub fn dsim(&self) -> DSIM_R {
        DSIM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - COLMUX"]
    #[inline(always)]
    pub fn colmux(&self) -> COLMUX_R {
        COLMUX_R::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bit 4 - TESRC"]
    #[inline(always)]
    pub fn tesrc(&self) -> TESRC_R {
        TESRC_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TEPOL"]
    #[inline(always)]
    pub fn tepol(&self) -> TEPOL_R {
        TEPOL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - AR"]
    #[inline(always)]
    pub fn ar(&self) -> AR_R {
        AR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - VSPOL"]
    #[inline(always)]
    pub fn vspol(&self) -> VSPOL_R {
        VSPOL_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DSIM"]
    #[inline(always)]
    #[must_use]
    pub fn dsim(&mut self) -> DSIM_W<DSI_WCFGR_SPEC, 0> {
        DSIM_W::new(self)
    }
    #[doc = "Bits 1:3 - COLMUX"]
    #[inline(always)]
    #[must_use]
    pub fn colmux(&mut self) -> COLMUX_W<DSI_WCFGR_SPEC, 1> {
        COLMUX_W::new(self)
    }
    #[doc = "Bit 4 - TESRC"]
    #[inline(always)]
    #[must_use]
    pub fn tesrc(&mut self) -> TESRC_W<DSI_WCFGR_SPEC, 4> {
        TESRC_W::new(self)
    }
    #[doc = "Bit 5 - TEPOL"]
    #[inline(always)]
    #[must_use]
    pub fn tepol(&mut self) -> TEPOL_W<DSI_WCFGR_SPEC, 5> {
        TEPOL_W::new(self)
    }
    #[doc = "Bit 6 - AR"]
    #[inline(always)]
    #[must_use]
    pub fn ar(&mut self) -> AR_W<DSI_WCFGR_SPEC, 6> {
        AR_W::new(self)
    }
    #[doc = "Bit 7 - VSPOL"]
    #[inline(always)]
    #[must_use]
    pub fn vspol(&mut self) -> VSPOL_W<DSI_WCFGR_SPEC, 7> {
        VSPOL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "DSI wrapper configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_wcfgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_wcfgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DSI_WCFGR_SPEC;
impl crate::RegisterSpec for DSI_WCFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_wcfgr::R`](R) reader structure"]
impl crate::Readable for DSI_WCFGR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dsi_wcfgr::W`](W) writer structure"]
impl crate::Writable for DSI_WCFGR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DSI_WCFGR to value 0"]
impl crate::Resettable for DSI_WCFGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
