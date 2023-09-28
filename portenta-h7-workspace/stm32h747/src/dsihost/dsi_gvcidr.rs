#[doc = "Register `DSI_GVCIDR` reader"]
pub type R = crate::R<DSI_GVCIDR_SPEC>;
#[doc = "Field `VCID` reader - VCID"]
pub type VCID_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:1 - VCID"]
    #[inline(always)]
    pub fn vcid(&self) -> VCID_R {
        VCID_R::new((self.bits & 3) as u8)
    }
}
#[doc = "DSI Host generic VCID register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_gvcidr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DSI_GVCIDR_SPEC;
impl crate::RegisterSpec for DSI_GVCIDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_gvcidr::R`](R) reader structure"]
impl crate::Readable for DSI_GVCIDR_SPEC {}
#[doc = "`reset()` method sets DSI_GVCIDR to value 0"]
impl crate::Resettable for DSI_GVCIDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
