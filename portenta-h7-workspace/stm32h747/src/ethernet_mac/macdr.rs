#[doc = "Register `MACDR` reader"]
pub type R = crate::R<MACDR_SPEC>;
#[doc = "Field `RPESTS` reader - RPESTS"]
pub type RPESTS_R = crate::BitReader;
#[doc = "Field `RFCFCSTS` reader - RFCFCSTS"]
pub type RFCFCSTS_R = crate::FieldReader;
#[doc = "Field `TPESTS` reader - TPESTS"]
pub type TPESTS_R = crate::BitReader;
#[doc = "Field `TFCSTS` reader - TFCSTS"]
pub type TFCSTS_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - RPESTS"]
    #[inline(always)]
    pub fn rpests(&self) -> RPESTS_R {
        RPESTS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - RFCFCSTS"]
    #[inline(always)]
    pub fn rfcfcsts(&self) -> RFCFCSTS_R {
        RFCFCSTS_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 16 - TPESTS"]
    #[inline(always)]
    pub fn tpests(&self) -> TPESTS_R {
        TPESTS_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:18 - TFCSTS"]
    #[inline(always)]
    pub fn tfcsts(&self) -> TFCSTS_R {
        TFCSTS_R::new(((self.bits >> 17) & 3) as u8)
    }
}
#[doc = "Debug register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macdr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACDR_SPEC;
impl crate::RegisterSpec for MACDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macdr::R`](R) reader structure"]
impl crate::Readable for MACDR_SPEC {}
#[doc = "`reset()` method sets MACDR to value 0"]
impl crate::Resettable for MACDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
