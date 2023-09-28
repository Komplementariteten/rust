#[doc = "Register `OTG_HS_DVBUSDIS` reader"]
pub type R = crate::R<OTG_HS_DVBUSDIS_SPEC>;
#[doc = "Register `OTG_HS_DVBUSDIS` writer"]
pub type W = crate::W<OTG_HS_DVBUSDIS_SPEC>;
#[doc = "Field `VBUSDT` reader - Device VBUS discharge time"]
pub type VBUSDT_R = crate::FieldReader<u16>;
#[doc = "Field `VBUSDT` writer - Device VBUS discharge time"]
pub type VBUSDT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - Device VBUS discharge time"]
    #[inline(always)]
    pub fn vbusdt(&self) -> VBUSDT_R {
        VBUSDT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Device VBUS discharge time"]
    #[inline(always)]
    #[must_use]
    pub fn vbusdt(&mut self) -> VBUSDT_W<OTG_HS_DVBUSDIS_SPEC, 0> {
        VBUSDT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "OTG_HS device VBUS discharge time register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hs_dvbusdis::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_hs_dvbusdis::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OTG_HS_DVBUSDIS_SPEC;
impl crate::RegisterSpec for OTG_HS_DVBUSDIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otg_hs_dvbusdis::R`](R) reader structure"]
impl crate::Readable for OTG_HS_DVBUSDIS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`otg_hs_dvbusdis::W`](W) writer structure"]
impl crate::Writable for OTG_HS_DVBUSDIS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OTG_HS_DVBUSDIS to value 0x17d7"]
impl crate::Resettable for OTG_HS_DVBUSDIS_SPEC {
    const RESET_VALUE: Self::Ux = 0x17d7;
}
