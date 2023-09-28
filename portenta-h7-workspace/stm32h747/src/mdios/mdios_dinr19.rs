#[doc = "Register `MDIOS_DINR19` reader"]
pub type R = crate::R<MDIOS_DINR19_SPEC>;
#[doc = "Field `DIN19` reader - Input data received from MDIO Master during write frames"]
pub type DIN19_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Input data received from MDIO Master during write frames"]
    #[inline(always)]
    pub fn din19(&self) -> DIN19_R {
        DIN19_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "MDIOS input data register 19\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdios_dinr19::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MDIOS_DINR19_SPEC;
impl crate::RegisterSpec for MDIOS_DINR19_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdios_dinr19::R`](R) reader structure"]
impl crate::Readable for MDIOS_DINR19_SPEC {}
#[doc = "`reset()` method sets MDIOS_DINR19 to value 0"]
impl crate::Resettable for MDIOS_DINR19_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
