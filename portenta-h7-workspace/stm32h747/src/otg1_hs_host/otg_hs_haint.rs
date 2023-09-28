#[doc = "Register `OTG_HS_HAINT` reader"]
pub type R = crate::R<OTG_HS_HAINT_SPEC>;
#[doc = "Field `HAINT` reader - Channel interrupts"]
pub type HAINT_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Channel interrupts"]
    #[inline(always)]
    pub fn haint(&self) -> HAINT_R {
        HAINT_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "OTG_HS Host all channels interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hs_haint::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OTG_HS_HAINT_SPEC;
impl crate::RegisterSpec for OTG_HS_HAINT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otg_hs_haint::R`](R) reader structure"]
impl crate::Readable for OTG_HS_HAINT_SPEC {}
#[doc = "`reset()` method sets OTG_HS_HAINT to value 0"]
impl crate::Resettable for OTG_HS_HAINT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
