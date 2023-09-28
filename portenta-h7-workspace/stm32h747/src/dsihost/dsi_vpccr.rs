#[doc = "Register `DSI_VPCCR` reader"]
pub type R = crate::R<DSI_VPCCR_SPEC>;
#[doc = "Field `VPSIZE` reader - VPSIZE"]
pub type VPSIZE_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:13 - VPSIZE"]
    #[inline(always)]
    pub fn vpsize(&self) -> VPSIZE_R {
        VPSIZE_R::new((self.bits & 0x3fff) as u16)
    }
}
#[doc = "DSI Host video packet current configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_vpccr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DSI_VPCCR_SPEC;
impl crate::RegisterSpec for DSI_VPCCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_vpccr::R`](R) reader structure"]
impl crate::Readable for DSI_VPCCR_SPEC {}
#[doc = "`reset()` method sets DSI_VPCCR to value 0"]
impl crate::Resettable for DSI_VPCCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
