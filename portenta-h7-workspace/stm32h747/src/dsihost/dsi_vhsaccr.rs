#[doc = "Register `DSI_VHSACCR` reader"]
pub type R = crate::R<DSI_VHSACCR_SPEC>;
#[doc = "Field `HSA` reader - HSA"]
pub type HSA_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:11 - HSA"]
    #[inline(always)]
    pub fn hsa(&self) -> HSA_R {
        HSA_R::new((self.bits & 0x0fff) as u16)
    }
}
#[doc = "DSI Host video HSA current configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_vhsaccr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DSI_VHSACCR_SPEC;
impl crate::RegisterSpec for DSI_VHSACCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_vhsaccr::R`](R) reader structure"]
impl crate::Readable for DSI_VHSACCR_SPEC {}
#[doc = "`reset()` method sets DSI_VHSACCR to value 0"]
impl crate::Resettable for DSI_VHSACCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
