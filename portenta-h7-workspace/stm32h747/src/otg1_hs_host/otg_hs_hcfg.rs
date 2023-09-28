#[doc = "Register `OTG_HS_HCFG` reader"]
pub type R = crate::R<OTG_HS_HCFG_SPEC>;
#[doc = "Register `OTG_HS_HCFG` writer"]
pub type W = crate::W<OTG_HS_HCFG_SPEC>;
#[doc = "Field `FSLSPCS` reader - FS/LS PHY clock select"]
pub type FSLSPCS_R = crate::FieldReader;
#[doc = "Field `FSLSPCS` writer - FS/LS PHY clock select"]
pub type FSLSPCS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `FSLSS` reader - FS- and LS-only support"]
pub type FSLSS_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:1 - FS/LS PHY clock select"]
    #[inline(always)]
    pub fn fslspcs(&self) -> FSLSPCS_R {
        FSLSPCS_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - FS- and LS-only support"]
    #[inline(always)]
    pub fn fslss(&self) -> FSLSS_R {
        FSLSS_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - FS/LS PHY clock select"]
    #[inline(always)]
    #[must_use]
    pub fn fslspcs(&mut self) -> FSLSPCS_W<OTG_HS_HCFG_SPEC, 0> {
        FSLSPCS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "OTG_HS host configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hs_hcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_hs_hcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OTG_HS_HCFG_SPEC;
impl crate::RegisterSpec for OTG_HS_HCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otg_hs_hcfg::R`](R) reader structure"]
impl crate::Readable for OTG_HS_HCFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`otg_hs_hcfg::W`](W) writer structure"]
impl crate::Writable for OTG_HS_HCFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OTG_HS_HCFG to value 0"]
impl crate::Resettable for OTG_HS_HCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
