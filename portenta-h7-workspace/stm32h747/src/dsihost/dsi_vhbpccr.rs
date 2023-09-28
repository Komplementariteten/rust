#[doc = "Register `DSI_VHBPCCR` reader"]
pub type R = crate::R<DSI_VHBPCCR_SPEC>;
#[doc = "Field `HBP` reader - HBP"]
pub type HBP_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:11 - HBP"]
    #[inline(always)]
    pub fn hbp(&self) -> HBP_R {
        HBP_R::new((self.bits & 0x0fff) as u16)
    }
}
#[doc = "DSI Host video HBP current configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_vhbpccr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DSI_VHBPCCR_SPEC;
impl crate::RegisterSpec for DSI_VHBPCCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_vhbpccr::R`](R) reader structure"]
impl crate::Readable for DSI_VHBPCCR_SPEC {}
#[doc = "`reset()` method sets DSI_VHBPCCR to value 0"]
impl crate::Resettable for DSI_VHBPCCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
