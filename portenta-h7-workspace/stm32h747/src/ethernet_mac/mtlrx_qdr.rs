#[doc = "Register `MTLRxQDR` reader"]
pub type R = crate::R<MTLRX_QDR_SPEC>;
#[doc = "Field `RWCSTS` reader - RWCSTS"]
pub type RWCSTS_R = crate::BitReader;
#[doc = "Field `RRCSTS` reader - RRCSTS"]
pub type RRCSTS_R = crate::FieldReader;
#[doc = "Field `RXQSTS` reader - RXQSTS"]
pub type RXQSTS_R = crate::FieldReader;
#[doc = "Field `PRXQ` reader - PRXQ"]
pub type PRXQ_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bit 0 - RWCSTS"]
    #[inline(always)]
    pub fn rwcsts(&self) -> RWCSTS_R {
        RWCSTS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - RRCSTS"]
    #[inline(always)]
    pub fn rrcsts(&self) -> RRCSTS_R {
        RRCSTS_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 4:5 - RXQSTS"]
    #[inline(always)]
    pub fn rxqsts(&self) -> RXQSTS_R {
        RXQSTS_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 16:29 - PRXQ"]
    #[inline(always)]
    pub fn prxq(&self) -> PRXQ_R {
        PRXQ_R::new(((self.bits >> 16) & 0x3fff) as u16)
    }
}
#[doc = "Rx queue debug register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mtlrx_qdr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MTLRX_QDR_SPEC;
impl crate::RegisterSpec for MTLRX_QDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mtlrx_qdr::R`](R) reader structure"]
impl crate::Readable for MTLRX_QDR_SPEC {}
#[doc = "`reset()` method sets MTLRxQDR to value 0"]
impl crate::Resettable for MTLRX_QDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
