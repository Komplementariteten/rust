#[doc = "Register `DSI_VLCCR` reader"]
pub type R = crate::R<DSI_VLCCR_SPEC>;
#[doc = "Field `HLINE` reader - HLINE"]
pub type HLINE_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:14 - HLINE"]
    #[inline(always)]
    pub fn hline(&self) -> HLINE_R {
        HLINE_R::new((self.bits & 0x7fff) as u16)
    }
}
#[doc = "DSI Host video line current configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_vlccr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DSI_VLCCR_SPEC;
impl crate::RegisterSpec for DSI_VLCCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_vlccr::R`](R) reader structure"]
impl crate::Readable for DSI_VLCCR_SPEC {}
#[doc = "`reset()` method sets DSI_VLCCR to value 0"]
impl crate::Resettable for DSI_VLCCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
