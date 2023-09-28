#[doc = "Register `MDIOS_DINR28` reader"]
pub type R = crate::R<MDIOS_DINR28_SPEC>;
#[doc = "Field `DIN28` reader - Input data received from MDIO Master during write frames"]
pub type DIN28_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Input data received from MDIO Master during write frames"]
    #[inline(always)]
    pub fn din28(&self) -> DIN28_R {
        DIN28_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "MDIOS input data register 28\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdios_dinr28::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MDIOS_DINR28_SPEC;
impl crate::RegisterSpec for MDIOS_DINR28_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdios_dinr28::R`](R) reader structure"]
impl crate::Readable for MDIOS_DINR28_SPEC {}
#[doc = "`reset()` method sets MDIOS_DINR28 to value 0"]
impl crate::Resettable for MDIOS_DINR28_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
