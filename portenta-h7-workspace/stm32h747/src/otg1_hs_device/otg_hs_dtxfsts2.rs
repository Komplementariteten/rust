#[doc = "Register `OTG_HS_DTXFSTS2` reader"]
pub type R = crate::R<OTG_HS_DTXFSTS2_SPEC>;
#[doc = "Field `INEPTFSAV` reader - IN endpoint TxFIFO space avail"]
pub type INEPTFSAV_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - IN endpoint TxFIFO space avail"]
    #[inline(always)]
    pub fn ineptfsav(&self) -> INEPTFSAV_R {
        INEPTFSAV_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "OTG_HS device IN endpoint transmit FIFO status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hs_dtxfsts2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OTG_HS_DTXFSTS2_SPEC;
impl crate::RegisterSpec for OTG_HS_DTXFSTS2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otg_hs_dtxfsts2::R`](R) reader structure"]
impl crate::Readable for OTG_HS_DTXFSTS2_SPEC {}
#[doc = "`reset()` method sets OTG_HS_DTXFSTS2 to value 0"]
impl crate::Resettable for OTG_HS_DTXFSTS2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
