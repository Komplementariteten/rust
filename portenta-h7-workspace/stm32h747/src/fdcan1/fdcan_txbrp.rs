#[doc = "Register `FDCAN_TXBRP` reader"]
pub type R = crate::R<FDCAN_TXBRP_SPEC>;
#[doc = "Field `TRP` reader - Transmission Request Pending"]
pub type TRP_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Transmission Request Pending"]
    #[inline(always)]
    pub fn trp(&self) -> TRP_R {
        TRP_R::new(self.bits)
    }
}
#[doc = "FDCAN Tx Buffer Request Pending Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_txbrp::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FDCAN_TXBRP_SPEC;
impl crate::RegisterSpec for FDCAN_TXBRP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_txbrp::R`](R) reader structure"]
impl crate::Readable for FDCAN_TXBRP_SPEC {}
#[doc = "`reset()` method sets FDCAN_TXBRP to value 0"]
impl crate::Resettable for FDCAN_TXBRP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
