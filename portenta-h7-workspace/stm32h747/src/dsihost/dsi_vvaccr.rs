#[doc = "Register `DSI_VVACCR` reader"]
pub type R = crate::R<DSI_VVACCR_SPEC>;
#[doc = "Field `VA` reader - VA"]
pub type VA_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:13 - VA"]
    #[inline(always)]
    pub fn va(&self) -> VA_R {
        VA_R::new((self.bits & 0x3fff) as u16)
    }
}
#[doc = "DSI Host video VA current configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_vvaccr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DSI_VVACCR_SPEC;
impl crate::RegisterSpec for DSI_VVACCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_vvaccr::R`](R) reader structure"]
impl crate::Readable for DSI_VVACCR_SPEC {}
#[doc = "`reset()` method sets DSI_VVACCR to value 0"]
impl crate::Resettable for DSI_VVACCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
