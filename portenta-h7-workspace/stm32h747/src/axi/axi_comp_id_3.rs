#[doc = "Register `AXI_COMP_ID_3` reader"]
pub type R = crate::R<AXI_COMP_ID_3_SPEC>;
#[doc = "Field `PREAMBLE` reader - Preamble bits 20 to 27"]
pub type PREAMBLE_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Preamble bits 20 to 27"]
    #[inline(always)]
    pub fn preamble(&self) -> PREAMBLE_R {
        PREAMBLE_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "AXI interconnect - component ID3 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`axi_comp_id_3::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AXI_COMP_ID_3_SPEC;
impl crate::RegisterSpec for AXI_COMP_ID_3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`axi_comp_id_3::R`](R) reader structure"]
impl crate::Readable for AXI_COMP_ID_3_SPEC {}
#[doc = "`reset()` method sets AXI_COMP_ID_3 to value 0x04"]
impl crate::Resettable for AXI_COMP_ID_3_SPEC {
    const RESET_VALUE: Self::Ux = 0x04;
}
