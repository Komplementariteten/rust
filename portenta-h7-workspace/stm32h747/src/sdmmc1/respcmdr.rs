#[doc = "Register `RESPCMDR` reader"]
pub type R = crate::R<RESPCMDR_SPEC>;
#[doc = "Field `RESPCMD` reader - Response command index"]
pub type RESPCMD_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:5 - Response command index"]
    #[inline(always)]
    pub fn respcmd(&self) -> RESPCMD_R {
        RESPCMD_R::new((self.bits & 0x3f) as u8)
    }
}
#[doc = "SDMMC command response register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`respcmdr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RESPCMDR_SPEC;
impl crate::RegisterSpec for RESPCMDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`respcmdr::R`](R) reader structure"]
impl crate::Readable for RESPCMDR_SPEC {}
#[doc = "`reset()` method sets RESPCMDR to value 0xa3c5_dd01"]
impl crate::Resettable for RESPCMDR_SPEC {
    const RESET_VALUE: Self::Ux = 0xa3c5_dd01;
}
