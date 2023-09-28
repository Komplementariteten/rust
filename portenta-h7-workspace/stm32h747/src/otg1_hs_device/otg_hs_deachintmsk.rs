#[doc = "Register `OTG_HS_DEACHINTMSK` reader"]
pub type R = crate::R<OTG_HS_DEACHINTMSK_SPEC>;
#[doc = "Register `OTG_HS_DEACHINTMSK` writer"]
pub type W = crate::W<OTG_HS_DEACHINTMSK_SPEC>;
#[doc = "Field `IEP1INTM` reader - IN Endpoint 1 interrupt mask bit"]
pub type IEP1INTM_R = crate::BitReader;
#[doc = "Field `IEP1INTM` writer - IN Endpoint 1 interrupt mask bit"]
pub type IEP1INTM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OEP1INTM` reader - OUT Endpoint 1 interrupt mask bit"]
pub type OEP1INTM_R = crate::BitReader;
#[doc = "Field `OEP1INTM` writer - OUT Endpoint 1 interrupt mask bit"]
pub type OEP1INTM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 1 - IN Endpoint 1 interrupt mask bit"]
    #[inline(always)]
    pub fn iep1intm(&self) -> IEP1INTM_R {
        IEP1INTM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 17 - OUT Endpoint 1 interrupt mask bit"]
    #[inline(always)]
    pub fn oep1intm(&self) -> OEP1INTM_R {
        OEP1INTM_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - IN Endpoint 1 interrupt mask bit"]
    #[inline(always)]
    #[must_use]
    pub fn iep1intm(&mut self) -> IEP1INTM_W<OTG_HS_DEACHINTMSK_SPEC, 1> {
        IEP1INTM_W::new(self)
    }
    #[doc = "Bit 17 - OUT Endpoint 1 interrupt mask bit"]
    #[inline(always)]
    #[must_use]
    pub fn oep1intm(&mut self) -> OEP1INTM_W<OTG_HS_DEACHINTMSK_SPEC, 17> {
        OEP1INTM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "OTG_HS device each endpoint interrupt register mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hs_deachintmsk::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_hs_deachintmsk::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OTG_HS_DEACHINTMSK_SPEC;
impl crate::RegisterSpec for OTG_HS_DEACHINTMSK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otg_hs_deachintmsk::R`](R) reader structure"]
impl crate::Readable for OTG_HS_DEACHINTMSK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`otg_hs_deachintmsk::W`](W) writer structure"]
impl crate::Writable for OTG_HS_DEACHINTMSK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OTG_HS_DEACHINTMSK to value 0"]
impl crate::Resettable for OTG_HS_DEACHINTMSK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
