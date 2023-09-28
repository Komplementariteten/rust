#[doc = "Register `AXI_COMP_ID_1` reader"]
pub type R = crate::R<AXI_COMP_ID_1_SPEC>;
#[doc = "Field `PREAMBLE` reader - Preamble bits 8 to 11"]
pub type PREAMBLE_R = crate::FieldReader;
#[doc = "Field `CLASS` reader - Component class"]
pub type CLASS_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Preamble bits 8 to 11"]
    #[inline(always)]
    pub fn preamble(&self) -> PREAMBLE_R {
        PREAMBLE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Component class"]
    #[inline(always)]
    pub fn class(&self) -> CLASS_R {
        CLASS_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "AXI interconnect - component ID1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`axi_comp_id_1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AXI_COMP_ID_1_SPEC;
impl crate::RegisterSpec for AXI_COMP_ID_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`axi_comp_id_1::R`](R) reader structure"]
impl crate::Readable for AXI_COMP_ID_1_SPEC {}
#[doc = "`reset()` method sets AXI_COMP_ID_1 to value 0x04"]
impl crate::Resettable for AXI_COMP_ID_1_SPEC {
    const RESET_VALUE: Self::Ux = 0x04;
}
