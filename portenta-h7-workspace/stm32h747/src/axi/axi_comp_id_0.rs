#[doc = "Register `AXI_COMP_ID_0` reader"]
pub type R = crate::R<AXI_COMP_ID_0_SPEC>;
#[doc = "Field `PREAMBLE` reader - Preamble bits 0 to 7"]
pub type PREAMBLE_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Preamble bits 0 to 7"]
    #[inline(always)]
    pub fn preamble(&self) -> PREAMBLE_R {
        PREAMBLE_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "AXI interconnect - component ID0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`axi_comp_id_0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AXI_COMP_ID_0_SPEC;
impl crate::RegisterSpec for AXI_COMP_ID_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`axi_comp_id_0::R`](R) reader structure"]
impl crate::Readable for AXI_COMP_ID_0_SPEC {}
#[doc = "`reset()` method sets AXI_COMP_ID_0 to value 0x04"]
impl crate::Resettable for AXI_COMP_ID_0_SPEC {
    const RESET_VALUE: Self::Ux = 0x04;
}
