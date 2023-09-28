#[doc = "Register `MDIOS_DINR9` reader"]
pub type R = crate::R<MDIOS_DINR9_SPEC>;
#[doc = "Field `DIN9` reader - Input data received from MDIO Master during write frames"]
pub type DIN9_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Input data received from MDIO Master during write frames"]
    #[inline(always)]
    pub fn din9(&self) -> DIN9_R {
        DIN9_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "MDIOS input data register 9\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdios_dinr9::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MDIOS_DINR9_SPEC;
impl crate::RegisterSpec for MDIOS_DINR9_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdios_dinr9::R`](R) reader structure"]
impl crate::Readable for MDIOS_DINR9_SPEC {}
#[doc = "`reset()` method sets MDIOS_DINR9 to value 0"]
impl crate::Resettable for MDIOS_DINR9_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
