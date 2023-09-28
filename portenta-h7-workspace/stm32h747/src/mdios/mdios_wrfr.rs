#[doc = "Register `MDIOS_WRFR` reader"]
pub type R = crate::R<MDIOS_WRFR_SPEC>;
#[doc = "Field `WRF` reader - Write flags for MDIO registers 0 to 31"]
pub type WRF_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Write flags for MDIO registers 0 to 31"]
    #[inline(always)]
    pub fn wrf(&self) -> WRF_R {
        WRF_R::new(self.bits)
    }
}
#[doc = "MDIOS write flag register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdios_wrfr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MDIOS_WRFR_SPEC;
impl crate::RegisterSpec for MDIOS_WRFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdios_wrfr::R`](R) reader structure"]
impl crate::Readable for MDIOS_WRFR_SPEC {}
#[doc = "`reset()` method sets MDIOS_WRFR to value 0"]
impl crate::Resettable for MDIOS_WRFR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
