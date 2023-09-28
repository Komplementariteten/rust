#[doc = "Register `UR13` reader"]
pub type R = crate::R<UR13_SPEC>;
#[doc = "Field `SDRS` reader - Secured DTCM RAM Size"]
pub type SDRS_R = crate::FieldReader;
#[doc = "Field `D1SBRST` reader - D1 Standby reset"]
pub type D1SBRST_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:1 - Secured DTCM RAM Size"]
    #[inline(always)]
    pub fn sdrs(&self) -> SDRS_R {
        SDRS_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 16 - D1 Standby reset"]
    #[inline(always)]
    pub fn d1sbrst(&self) -> D1SBRST_R {
        D1SBRST_R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[doc = "SYSCFG user register 13\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ur13::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UR13_SPEC;
impl crate::RegisterSpec for UR13_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ur13::R`](R) reader structure"]
impl crate::Readable for UR13_SPEC {}
#[doc = "`reset()` method sets UR13 to value 0"]
impl crate::Resettable for UR13_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
