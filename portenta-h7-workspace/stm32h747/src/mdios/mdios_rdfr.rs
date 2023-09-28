#[doc = "Register `MDIOS_RDFR` reader"]
pub type R = crate::R<MDIOS_RDFR_SPEC>;
#[doc = "Field `RDF` reader - Read flags for MDIO registers 0 to 31"]
pub type RDF_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Read flags for MDIO registers 0 to 31"]
    #[inline(always)]
    pub fn rdf(&self) -> RDF_R {
        RDF_R::new(self.bits)
    }
}
#[doc = "MDIOS read flag register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdios_rdfr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MDIOS_RDFR_SPEC;
impl crate::RegisterSpec for MDIOS_RDFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdios_rdfr::R`](R) reader structure"]
impl crate::Readable for MDIOS_RDFR_SPEC {}
#[doc = "`reset()` method sets MDIOS_RDFR to value 0"]
impl crate::Resettable for MDIOS_RDFR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
