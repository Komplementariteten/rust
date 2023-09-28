#[doc = "Register `DSI_VNPCCR` reader"]
pub type R = crate::R<DSI_VNPCCR_SPEC>;
#[doc = "Field `NPSIZE` reader - NPSIZE"]
pub type NPSIZE_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:12 - NPSIZE"]
    #[inline(always)]
    pub fn npsize(&self) -> NPSIZE_R {
        NPSIZE_R::new((self.bits & 0x1fff) as u16)
    }
}
#[doc = "DSI Host video null packet current configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_vnpccr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DSI_VNPCCR_SPEC;
impl crate::RegisterSpec for DSI_VNPCCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_vnpccr::R`](R) reader structure"]
impl crate::Readable for DSI_VNPCCR_SPEC {}
#[doc = "`reset()` method sets DSI_VNPCCR to value 0"]
impl crate::Resettable for DSI_VNPCCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
