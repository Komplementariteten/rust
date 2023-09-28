#[doc = "Register `MDIOS_DINR21` reader"]
pub type R = crate::R<MDIOS_DINR21_SPEC>;
#[doc = "Field `DIN21` reader - Input data received from MDIO Master during write frames"]
pub type DIN21_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Input data received from MDIO Master during write frames"]
    #[inline(always)]
    pub fn din21(&self) -> DIN21_R {
        DIN21_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "MDIOS input data register 21\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdios_dinr21::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MDIOS_DINR21_SPEC;
impl crate::RegisterSpec for MDIOS_DINR21_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdios_dinr21::R`](R) reader structure"]
impl crate::Readable for MDIOS_DINR21_SPEC {}
#[doc = "`reset()` method sets MDIOS_DINR21 to value 0"]
impl crate::Resettable for MDIOS_DINR21_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
