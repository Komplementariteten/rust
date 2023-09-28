#[doc = "Register `MDIOS_DINR12` reader"]
pub type R = crate::R<MDIOS_DINR12_SPEC>;
#[doc = "Field `DIN12` reader - Input data received from MDIO Master during write frames"]
pub type DIN12_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Input data received from MDIO Master during write frames"]
    #[inline(always)]
    pub fn din12(&self) -> DIN12_R {
        DIN12_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "MDIOS input data register 12\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdios_dinr12::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MDIOS_DINR12_SPEC;
impl crate::RegisterSpec for MDIOS_DINR12_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdios_dinr12::R`](R) reader structure"]
impl crate::Readable for MDIOS_DINR12_SPEC {}
#[doc = "`reset()` method sets MDIOS_DINR12 to value 0"]
impl crate::Resettable for MDIOS_DINR12_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
