#[doc = "Register `FDCAN_TTCSM` reader"]
pub type R = crate::R<FDCAN_TTCSM_SPEC>;
#[doc = "Field `CSM` reader - Cycle Sync Mark"]
pub type CSM_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Cycle Sync Mark"]
    #[inline(always)]
    pub fn csm(&self) -> CSM_R {
        CSM_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "FDCAN TT Cycle Sync Mark Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_ttcsm::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FDCAN_TTCSM_SPEC;
impl crate::RegisterSpec for FDCAN_TTCSM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_ttcsm::R`](R) reader structure"]
impl crate::Readable for FDCAN_TTCSM_SPEC {}
#[doc = "`reset()` method sets FDCAN_TTCSM to value 0"]
impl crate::Resettable for FDCAN_TTCSM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
