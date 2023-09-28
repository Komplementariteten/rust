#[doc = "Register `DSI_VR` reader"]
pub type R = crate::R<DSI_VR_SPEC>;
#[doc = "Field `VERSION` reader - VERSION"]
pub type VERSION_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - VERSION"]
    #[inline(always)]
    pub fn version(&self) -> VERSION_R {
        VERSION_R::new(self.bits)
    }
}
#[doc = "DSI Host version register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_vr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DSI_VR_SPEC;
impl crate::RegisterSpec for DSI_VR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_vr::R`](R) reader structure"]
impl crate::Readable for DSI_VR_SPEC {}
#[doc = "`reset()` method sets DSI_VR to value 0x3133_302a"]
impl crate::Resettable for DSI_VR_SPEC {
    const RESET_VALUE: Self::Ux = 0x3133_302a;
}
