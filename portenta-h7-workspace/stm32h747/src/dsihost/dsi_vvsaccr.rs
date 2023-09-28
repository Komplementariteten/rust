#[doc = "Register `DSI_VVSACCR` reader"]
pub type R = crate::R<DSI_VVSACCR_SPEC>;
#[doc = "Field `VSA` reader - VSA"]
pub type VSA_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:9 - VSA"]
    #[inline(always)]
    pub fn vsa(&self) -> VSA_R {
        VSA_R::new((self.bits & 0x03ff) as u16)
    }
}
#[doc = "DSI Host video VSA current configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_vvsaccr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DSI_VVSACCR_SPEC;
impl crate::RegisterSpec for DSI_VVSACCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_vvsaccr::R`](R) reader structure"]
impl crate::Readable for DSI_VVSACCR_SPEC {}
#[doc = "`reset()` method sets DSI_VVSACCR to value 0"]
impl crate::Resettable for DSI_VVSACCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
