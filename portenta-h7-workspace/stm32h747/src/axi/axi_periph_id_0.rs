#[doc = "Register `AXI_PERIPH_ID_0` reader"]
pub type R = crate::R<AXI_PERIPH_ID_0_SPEC>;
#[doc = "Field `PARTNUM` reader - Peripheral part number bits 0 to 7"]
pub type PARTNUM_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Peripheral part number bits 0 to 7"]
    #[inline(always)]
    pub fn partnum(&self) -> PARTNUM_R {
        PARTNUM_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "AXI interconnect - peripheral ID0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`axi_periph_id_0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AXI_PERIPH_ID_0_SPEC;
impl crate::RegisterSpec for AXI_PERIPH_ID_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`axi_periph_id_0::R`](R) reader structure"]
impl crate::Readable for AXI_PERIPH_ID_0_SPEC {}
#[doc = "`reset()` method sets AXI_PERIPH_ID_0 to value 0x04"]
impl crate::Resettable for AXI_PERIPH_ID_0_SPEC {
    const RESET_VALUE: Self::Ux = 0x04;
}
