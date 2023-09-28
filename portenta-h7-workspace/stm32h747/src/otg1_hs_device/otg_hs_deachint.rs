#[doc = "Register `OTG_HS_DEACHINT` reader"]
pub type R = crate::R<OTG_HS_DEACHINT_SPEC>;
#[doc = "Register `OTG_HS_DEACHINT` writer"]
pub type W = crate::W<OTG_HS_DEACHINT_SPEC>;
#[doc = "Field `IEP1INT` reader - IN endpoint 1interrupt bit"]
pub type IEP1INT_R = crate::BitReader;
#[doc = "Field `IEP1INT` writer - IN endpoint 1interrupt bit"]
pub type IEP1INT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OEP1INT` reader - OUT endpoint 1 interrupt bit"]
pub type OEP1INT_R = crate::BitReader;
#[doc = "Field `OEP1INT` writer - OUT endpoint 1 interrupt bit"]
pub type OEP1INT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 1 - IN endpoint 1interrupt bit"]
    #[inline(always)]
    pub fn iep1int(&self) -> IEP1INT_R {
        IEP1INT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 17 - OUT endpoint 1 interrupt bit"]
    #[inline(always)]
    pub fn oep1int(&self) -> OEP1INT_R {
        OEP1INT_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - IN endpoint 1interrupt bit"]
    #[inline(always)]
    #[must_use]
    pub fn iep1int(&mut self) -> IEP1INT_W<OTG_HS_DEACHINT_SPEC, 1> {
        IEP1INT_W::new(self)
    }
    #[doc = "Bit 17 - OUT endpoint 1 interrupt bit"]
    #[inline(always)]
    #[must_use]
    pub fn oep1int(&mut self) -> OEP1INT_W<OTG_HS_DEACHINT_SPEC, 17> {
        OEP1INT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "OTG_HS device each endpoint interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hs_deachint::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_hs_deachint::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OTG_HS_DEACHINT_SPEC;
impl crate::RegisterSpec for OTG_HS_DEACHINT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otg_hs_deachint::R`](R) reader structure"]
impl crate::Readable for OTG_HS_DEACHINT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`otg_hs_deachint::W`](W) writer structure"]
impl crate::Writable for OTG_HS_DEACHINT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OTG_HS_DEACHINT to value 0"]
impl crate::Resettable for OTG_HS_DEACHINT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
