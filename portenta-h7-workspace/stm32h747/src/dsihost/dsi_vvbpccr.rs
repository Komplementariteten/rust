#[doc = "Register `DSI_VVBPCCR` reader"]
pub type R = crate::R<DSI_VVBPCCR_SPEC>;
#[doc = "Field `VBP` reader - VBP"]
pub type VBP_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:9 - VBP"]
    #[inline(always)]
    pub fn vbp(&self) -> VBP_R {
        VBP_R::new((self.bits & 0x03ff) as u16)
    }
}
#[doc = "DSI Host video VBP current configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_vvbpccr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DSI_VVBPCCR_SPEC;
impl crate::RegisterSpec for DSI_VVBPCCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_vvbpccr::R`](R) reader structure"]
impl crate::Readable for DSI_VVBPCCR_SPEC {}
#[doc = "`reset()` method sets DSI_VVBPCCR to value 0"]
impl crate::Resettable for DSI_VVBPCCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
