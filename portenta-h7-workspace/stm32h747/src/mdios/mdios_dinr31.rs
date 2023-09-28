#[doc = "Register `MDIOS_DINR31` reader"]
pub type R = crate::R<MDIOS_DINR31_SPEC>;
#[doc = "Field `DIN31` reader - Input data received from MDIO Master during write frames"]
pub type DIN31_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Input data received from MDIO Master during write frames"]
    #[inline(always)]
    pub fn din31(&self) -> DIN31_R {
        DIN31_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "MDIOS input data register 31\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdios_dinr31::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MDIOS_DINR31_SPEC;
impl crate::RegisterSpec for MDIOS_DINR31_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdios_dinr31::R`](R) reader structure"]
impl crate::Readable for MDIOS_DINR31_SPEC {}
#[doc = "`reset()` method sets MDIOS_DINR31 to value 0"]
impl crate::Resettable for MDIOS_DINR31_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
