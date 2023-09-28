#[doc = "Register `OTG_HS_HAINTMSK` reader"]
pub type R = crate::R<OTG_HS_HAINTMSK_SPEC>;
#[doc = "Register `OTG_HS_HAINTMSK` writer"]
pub type W = crate::W<OTG_HS_HAINTMSK_SPEC>;
#[doc = "Field `HAINTM` reader - Channel interrupt mask"]
pub type HAINTM_R = crate::FieldReader<u16>;
#[doc = "Field `HAINTM` writer - Channel interrupt mask"]
pub type HAINTM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - Channel interrupt mask"]
    #[inline(always)]
    pub fn haintm(&self) -> HAINTM_R {
        HAINTM_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Channel interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn haintm(&mut self) -> HAINTM_W<OTG_HS_HAINTMSK_SPEC, 0> {
        HAINTM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "OTG_HS host all channels interrupt mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hs_haintmsk::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_hs_haintmsk::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OTG_HS_HAINTMSK_SPEC;
impl crate::RegisterSpec for OTG_HS_HAINTMSK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otg_hs_haintmsk::R`](R) reader structure"]
impl crate::Readable for OTG_HS_HAINTMSK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`otg_hs_haintmsk::W`](W) writer structure"]
impl crate::Writable for OTG_HS_HAINTMSK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OTG_HS_HAINTMSK to value 0"]
impl crate::Resettable for OTG_HS_HAINTMSK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
