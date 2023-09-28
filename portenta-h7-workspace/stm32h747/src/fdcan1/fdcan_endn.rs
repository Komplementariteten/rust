#[doc = "Register `FDCAN_ENDN` reader"]
pub type R = crate::R<FDCAN_ENDN_SPEC>;
#[doc = "Field `ETV` reader - Endiannes Test Value"]
pub type ETV_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Endiannes Test Value"]
    #[inline(always)]
    pub fn etv(&self) -> ETV_R {
        ETV_R::new(self.bits)
    }
}
#[doc = "FDCAN Core Release Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_endn::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FDCAN_ENDN_SPEC;
impl crate::RegisterSpec for FDCAN_ENDN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_endn::R`](R) reader structure"]
impl crate::Readable for FDCAN_ENDN_SPEC {}
#[doc = "`reset()` method sets FDCAN_ENDN to value 0"]
impl crate::Resettable for FDCAN_ENDN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
