#[doc = "Register `OTG_HS_DVBUSPULSE` reader"]
pub type R = crate::R<OTG_HS_DVBUSPULSE_SPEC>;
#[doc = "Register `OTG_HS_DVBUSPULSE` writer"]
pub type W = crate::W<OTG_HS_DVBUSPULSE_SPEC>;
#[doc = "Field `DVBUSP` reader - Device VBUS pulsing time"]
pub type DVBUSP_R = crate::FieldReader<u16>;
#[doc = "Field `DVBUSP` writer - Device VBUS pulsing time"]
pub type DVBUSP_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 12, O, u16>;
impl R {
    #[doc = "Bits 0:11 - Device VBUS pulsing time"]
    #[inline(always)]
    pub fn dvbusp(&self) -> DVBUSP_R {
        DVBUSP_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Device VBUS pulsing time"]
    #[inline(always)]
    #[must_use]
    pub fn dvbusp(&mut self) -> DVBUSP_W<OTG_HS_DVBUSPULSE_SPEC, 0> {
        DVBUSP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "OTG_HS device VBUS pulsing time register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hs_dvbuspulse::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_hs_dvbuspulse::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OTG_HS_DVBUSPULSE_SPEC;
impl crate::RegisterSpec for OTG_HS_DVBUSPULSE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otg_hs_dvbuspulse::R`](R) reader structure"]
impl crate::Readable for OTG_HS_DVBUSPULSE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`otg_hs_dvbuspulse::W`](W) writer structure"]
impl crate::Writable for OTG_HS_DVBUSPULSE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OTG_HS_DVBUSPULSE to value 0x05b8"]
impl crate::Resettable for OTG_HS_DVBUSPULSE_SPEC {
    const RESET_VALUE: Self::Ux = 0x05b8;
}
