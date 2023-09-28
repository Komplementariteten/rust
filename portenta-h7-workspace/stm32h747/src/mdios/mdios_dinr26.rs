#[doc = "Register `MDIOS_DINR26` reader"]
pub type R = crate::R<MDIOS_DINR26_SPEC>;
#[doc = "Field `DIN26` reader - Input data received from MDIO Master during write frames"]
pub type DIN26_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Input data received from MDIO Master during write frames"]
    #[inline(always)]
    pub fn din26(&self) -> DIN26_R {
        DIN26_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "MDIOS input data register 26\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdios_dinr26::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MDIOS_DINR26_SPEC;
impl crate::RegisterSpec for MDIOS_DINR26_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdios_dinr26::R`](R) reader structure"]
impl crate::Readable for MDIOS_DINR26_SPEC {}
#[doc = "`reset()` method sets MDIOS_DINR26 to value 0"]
impl crate::Resettable for MDIOS_DINR26_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
