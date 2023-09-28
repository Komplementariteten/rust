#[doc = "Register `DSI_WPCR3` reader"]
pub type R = crate::R<DSI_WPCR3_SPEC>;
#[doc = "Register `DSI_WPCR3` writer"]
pub type W = crate::W<DSI_WPCR3_SPEC>;
#[doc = "Field `THSZERO` reader - THSZERO"]
pub type THSZERO_R = crate::FieldReader;
#[doc = "Field `THSZERO` writer - THSZERO"]
pub type THSZERO_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `TLPXD` reader - TLPXD"]
pub type TLPXD_R = crate::FieldReader;
#[doc = "Field `TLPXD` writer - TLPXD"]
pub type TLPXD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `THSEXIT` reader - THSEXIT"]
pub type THSEXIT_R = crate::FieldReader;
#[doc = "Field `THSEXIT` writer - THSEXIT"]
pub type THSEXIT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `TLPXC` reader - TLPXC"]
pub type TLPXC_R = crate::FieldReader;
#[doc = "Field `TLPXC` writer - TLPXC"]
pub type TLPXC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - THSZERO"]
    #[inline(always)]
    pub fn thszero(&self) -> THSZERO_R {
        THSZERO_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - TLPXD"]
    #[inline(always)]
    pub fn tlpxd(&self) -> TLPXD_R {
        TLPXD_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - THSEXIT"]
    #[inline(always)]
    pub fn thsexit(&self) -> THSEXIT_R {
        THSEXIT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - TLPXC"]
    #[inline(always)]
    pub fn tlpxc(&self) -> TLPXC_R {
        TLPXC_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - THSZERO"]
    #[inline(always)]
    #[must_use]
    pub fn thszero(&mut self) -> THSZERO_W<DSI_WPCR3_SPEC, 0> {
        THSZERO_W::new(self)
    }
    #[doc = "Bits 8:15 - TLPXD"]
    #[inline(always)]
    #[must_use]
    pub fn tlpxd(&mut self) -> TLPXD_W<DSI_WPCR3_SPEC, 8> {
        TLPXD_W::new(self)
    }
    #[doc = "Bits 16:23 - THSEXIT"]
    #[inline(always)]
    #[must_use]
    pub fn thsexit(&mut self) -> THSEXIT_W<DSI_WPCR3_SPEC, 16> {
        THSEXIT_W::new(self)
    }
    #[doc = "Bits 24:31 - TLPXC"]
    #[inline(always)]
    #[must_use]
    pub fn tlpxc(&mut self) -> TLPXC_W<DSI_WPCR3_SPEC, 24> {
        TLPXC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "DSI wrapper PHY configuration register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_wpcr3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_wpcr3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DSI_WPCR3_SPEC;
impl crate::RegisterSpec for DSI_WPCR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_wpcr3::R`](R) reader structure"]
impl crate::Readable for DSI_WPCR3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dsi_wpcr3::W`](W) writer structure"]
impl crate::Writable for DSI_WPCR3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DSI_WPCR3 to value 0"]
impl crate::Resettable for DSI_WPCR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
