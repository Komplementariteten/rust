#[doc = "Register `OTG_HS_DOEPTSIZ0` reader"]
pub type R = crate::R<OTG_HS_DOEPTSIZ0_SPEC>;
#[doc = "Register `OTG_HS_DOEPTSIZ0` writer"]
pub type W = crate::W<OTG_HS_DOEPTSIZ0_SPEC>;
#[doc = "Field `XFRSIZ` reader - Transfer size"]
pub type XFRSIZ_R = crate::FieldReader;
#[doc = "Field `XFRSIZ` writer - Transfer size"]
pub type XFRSIZ_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
#[doc = "Field `PKTCNT` reader - Packet count"]
pub type PKTCNT_R = crate::BitReader;
#[doc = "Field `PKTCNT` writer - Packet count"]
pub type PKTCNT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `STUPCNT` reader - SETUP packet count"]
pub type STUPCNT_R = crate::FieldReader;
#[doc = "Field `STUPCNT` writer - SETUP packet count"]
pub type STUPCNT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
impl R {
    #[doc = "Bits 0:6 - Transfer size"]
    #[inline(always)]
    pub fn xfrsiz(&self) -> XFRSIZ_R {
        XFRSIZ_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 19 - Packet count"]
    #[inline(always)]
    pub fn pktcnt(&self) -> PKTCNT_R {
        PKTCNT_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 29:30 - SETUP packet count"]
    #[inline(always)]
    pub fn stupcnt(&self) -> STUPCNT_R {
        STUPCNT_R::new(((self.bits >> 29) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Transfer size"]
    #[inline(always)]
    #[must_use]
    pub fn xfrsiz(&mut self) -> XFRSIZ_W<OTG_HS_DOEPTSIZ0_SPEC, 0> {
        XFRSIZ_W::new(self)
    }
    #[doc = "Bit 19 - Packet count"]
    #[inline(always)]
    #[must_use]
    pub fn pktcnt(&mut self) -> PKTCNT_W<OTG_HS_DOEPTSIZ0_SPEC, 19> {
        PKTCNT_W::new(self)
    }
    #[doc = "Bits 29:30 - SETUP packet count"]
    #[inline(always)]
    #[must_use]
    pub fn stupcnt(&mut self) -> STUPCNT_W<OTG_HS_DOEPTSIZ0_SPEC, 29> {
        STUPCNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "OTG_HS device endpoint-0 transfer size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hs_doeptsiz0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_hs_doeptsiz0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OTG_HS_DOEPTSIZ0_SPEC;
impl crate::RegisterSpec for OTG_HS_DOEPTSIZ0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otg_hs_doeptsiz0::R`](R) reader structure"]
impl crate::Readable for OTG_HS_DOEPTSIZ0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`otg_hs_doeptsiz0::W`](W) writer structure"]
impl crate::Writable for OTG_HS_DOEPTSIZ0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OTG_HS_DOEPTSIZ0 to value 0"]
impl crate::Resettable for OTG_HS_DOEPTSIZ0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
