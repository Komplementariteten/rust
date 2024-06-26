#[doc = "Register `MDIOS_DINR14` reader"]
pub type R = crate::R<MDIOS_DINR14_SPEC>;
#[doc = "Field `DIN14` reader - Input data received from MDIO Master during write frames"]
pub type DIN14_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Input data received from MDIO Master during write frames"]
    #[inline(always)]
    pub fn din14(&self) -> DIN14_R {
        DIN14_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "MDIOS input data register 14\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdios_dinr14::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MDIOS_DINR14_SPEC;
impl crate::RegisterSpec for MDIOS_DINR14_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdios_dinr14::R`](R) reader structure"]
impl crate::Readable for MDIOS_DINR14_SPEC {}
#[doc = "`reset()` method sets MDIOS_DINR14 to value 0"]
impl crate::Resettable for MDIOS_DINR14_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
