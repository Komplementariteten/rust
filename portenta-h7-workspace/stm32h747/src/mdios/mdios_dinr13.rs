#[doc = "Register `MDIOS_DINR13` reader"]
pub type R = crate::R<MDIOS_DINR13_SPEC>;
#[doc = "Field `DIN13` reader - Input data received from MDIO Master during write frames"]
pub type DIN13_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Input data received from MDIO Master during write frames"]
    #[inline(always)]
    pub fn din13(&self) -> DIN13_R {
        DIN13_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "MDIOS input data register 13\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdios_dinr13::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MDIOS_DINR13_SPEC;
impl crate::RegisterSpec for MDIOS_DINR13_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdios_dinr13::R`](R) reader structure"]
impl crate::Readable for MDIOS_DINR13_SPEC {}
#[doc = "`reset()` method sets MDIOS_DINR13 to value 0"]
impl crate::Resettable for MDIOS_DINR13_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
