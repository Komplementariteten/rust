#[doc = "Register `OTG_HS_HCSPLT13` reader"]
pub type R = crate::R<OTG_HS_HCSPLT13_SPEC>;
#[doc = "Register `OTG_HS_HCSPLT13` writer"]
pub type W = crate::W<OTG_HS_HCSPLT13_SPEC>;
#[doc = "Field `PRTADDR` reader - Port address"]
pub type PRTADDR_R = crate::FieldReader;
#[doc = "Field `PRTADDR` writer - Port address"]
pub type PRTADDR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
#[doc = "Field `HUBADDR` reader - Hub address"]
pub type HUBADDR_R = crate::FieldReader;
#[doc = "Field `HUBADDR` writer - Hub address"]
pub type HUBADDR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
#[doc = "Field `XACTPOS` reader - XACTPOS"]
pub type XACTPOS_R = crate::FieldReader;
#[doc = "Field `XACTPOS` writer - XACTPOS"]
pub type XACTPOS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `COMPLSPLT` reader - Do complete split"]
pub type COMPLSPLT_R = crate::BitReader;
#[doc = "Field `COMPLSPLT` writer - Do complete split"]
pub type COMPLSPLT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SPLITEN` reader - Split enable"]
pub type SPLITEN_R = crate::BitReader;
#[doc = "Field `SPLITEN` writer - Split enable"]
pub type SPLITEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:6 - Port address"]
    #[inline(always)]
    pub fn prtaddr(&self) -> PRTADDR_R {
        PRTADDR_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 7:13 - Hub address"]
    #[inline(always)]
    pub fn hubaddr(&self) -> HUBADDR_R {
        HUBADDR_R::new(((self.bits >> 7) & 0x7f) as u8)
    }
    #[doc = "Bits 14:15 - XACTPOS"]
    #[inline(always)]
    pub fn xactpos(&self) -> XACTPOS_R {
        XACTPOS_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 16 - Do complete split"]
    #[inline(always)]
    pub fn complsplt(&self) -> COMPLSPLT_R {
        COMPLSPLT_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 31 - Split enable"]
    #[inline(always)]
    pub fn spliten(&self) -> SPLITEN_R {
        SPLITEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Port address"]
    #[inline(always)]
    #[must_use]
    pub fn prtaddr(&mut self) -> PRTADDR_W<OTG_HS_HCSPLT13_SPEC, 0> {
        PRTADDR_W::new(self)
    }
    #[doc = "Bits 7:13 - Hub address"]
    #[inline(always)]
    #[must_use]
    pub fn hubaddr(&mut self) -> HUBADDR_W<OTG_HS_HCSPLT13_SPEC, 7> {
        HUBADDR_W::new(self)
    }
    #[doc = "Bits 14:15 - XACTPOS"]
    #[inline(always)]
    #[must_use]
    pub fn xactpos(&mut self) -> XACTPOS_W<OTG_HS_HCSPLT13_SPEC, 14> {
        XACTPOS_W::new(self)
    }
    #[doc = "Bit 16 - Do complete split"]
    #[inline(always)]
    #[must_use]
    pub fn complsplt(&mut self) -> COMPLSPLT_W<OTG_HS_HCSPLT13_SPEC, 16> {
        COMPLSPLT_W::new(self)
    }
    #[doc = "Bit 31 - Split enable"]
    #[inline(always)]
    #[must_use]
    pub fn spliten(&mut self) -> SPLITEN_W<OTG_HS_HCSPLT13_SPEC, 31> {
        SPLITEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "OTG_HS host channel-13 split control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hs_hcsplt13::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_hs_hcsplt13::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OTG_HS_HCSPLT13_SPEC;
impl crate::RegisterSpec for OTG_HS_HCSPLT13_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otg_hs_hcsplt13::R`](R) reader structure"]
impl crate::Readable for OTG_HS_HCSPLT13_SPEC {}
#[doc = "`write(|w| ..)` method takes [`otg_hs_hcsplt13::W`](W) writer structure"]
impl crate::Writable for OTG_HS_HCSPLT13_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OTG_HS_HCSPLT13 to value 0"]
impl crate::Resettable for OTG_HS_HCSPLT13_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
