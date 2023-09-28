#[doc = "Register `DSI_VCCCR` reader"]
pub type R = crate::R<DSI_VCCCR_SPEC>;
#[doc = "Field `NUMC` reader - NUMC"]
pub type NUMC_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:12 - NUMC"]
    #[inline(always)]
    pub fn numc(&self) -> NUMC_R {
        NUMC_R::new((self.bits & 0x1fff) as u16)
    }
}
#[doc = "DSI Host video chunks current configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_vcccr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DSI_VCCCR_SPEC;
impl crate::RegisterSpec for DSI_VCCCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_vcccr::R`](R) reader structure"]
impl crate::Readable for DSI_VCCCR_SPEC {}
#[doc = "`reset()` method sets DSI_VCCCR to value 0"]
impl crate::Resettable for DSI_VCCCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
