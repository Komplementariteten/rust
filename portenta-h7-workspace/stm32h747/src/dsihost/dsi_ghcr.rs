#[doc = "Register `DSI_GHCR` reader"]
pub type R = crate::R<DSI_GHCR_SPEC>;
#[doc = "Register `DSI_GHCR` writer"]
pub type W = crate::W<DSI_GHCR_SPEC>;
#[doc = "Field `DT` reader - DT"]
pub type DT_R = crate::FieldReader;
#[doc = "Field `DT` writer - DT"]
pub type DT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
#[doc = "Field `VCID` reader - VCID"]
pub type VCID_R = crate::FieldReader;
#[doc = "Field `VCID` writer - VCID"]
pub type VCID_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `WCLSB` reader - WCLSB"]
pub type WCLSB_R = crate::FieldReader;
#[doc = "Field `WCLSB` writer - WCLSB"]
pub type WCLSB_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `WCMSB` reader - WCMSB"]
pub type WCMSB_R = crate::FieldReader;
#[doc = "Field `WCMSB` writer - WCMSB"]
pub type WCMSB_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:5 - DT"]
    #[inline(always)]
    pub fn dt(&self) -> DT_R {
        DT_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:7 - VCID"]
    #[inline(always)]
    pub fn vcid(&self) -> VCID_R {
        VCID_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:15 - WCLSB"]
    #[inline(always)]
    pub fn wclsb(&self) -> WCLSB_R {
        WCLSB_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - WCMSB"]
    #[inline(always)]
    pub fn wcmsb(&self) -> WCMSB_R {
        WCMSB_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - DT"]
    #[inline(always)]
    #[must_use]
    pub fn dt(&mut self) -> DT_W<DSI_GHCR_SPEC, 0> {
        DT_W::new(self)
    }
    #[doc = "Bits 6:7 - VCID"]
    #[inline(always)]
    #[must_use]
    pub fn vcid(&mut self) -> VCID_W<DSI_GHCR_SPEC, 6> {
        VCID_W::new(self)
    }
    #[doc = "Bits 8:15 - WCLSB"]
    #[inline(always)]
    #[must_use]
    pub fn wclsb(&mut self) -> WCLSB_W<DSI_GHCR_SPEC, 8> {
        WCLSB_W::new(self)
    }
    #[doc = "Bits 16:23 - WCMSB"]
    #[inline(always)]
    #[must_use]
    pub fn wcmsb(&mut self) -> WCMSB_W<DSI_GHCR_SPEC, 16> {
        WCMSB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "DSI Host generic header configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_ghcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_ghcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DSI_GHCR_SPEC;
impl crate::RegisterSpec for DSI_GHCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_ghcr::R`](R) reader structure"]
impl crate::Readable for DSI_GHCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dsi_ghcr::W`](W) writer structure"]
impl crate::Writable for DSI_GHCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DSI_GHCR to value 0"]
impl crate::Resettable for DSI_GHCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
