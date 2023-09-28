#[doc = "Register `AXI_PERIPH_ID_4` reader"]
pub type R = crate::R<AXI_PERIPH_ID_4_SPEC>;
#[doc = "Field `JEP106CON` reader - JEP106 continuation code"]
pub type JEP106CON_R = crate::FieldReader;
#[doc = "Field `KCOUNT4` reader - Register file size"]
pub type KCOUNT4_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - JEP106 continuation code"]
    #[inline(always)]
    pub fn jep106con(&self) -> JEP106CON_R {
        JEP106CON_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Register file size"]
    #[inline(always)]
    pub fn kcount4(&self) -> KCOUNT4_R {
        KCOUNT4_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "AXI interconnect - peripheral ID4 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`axi_periph_id_4::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AXI_PERIPH_ID_4_SPEC;
impl crate::RegisterSpec for AXI_PERIPH_ID_4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`axi_periph_id_4::R`](R) reader structure"]
impl crate::Readable for AXI_PERIPH_ID_4_SPEC {}
#[doc = "`reset()` method sets AXI_PERIPH_ID_4 to value 0x04"]
impl crate::Resettable for AXI_PERIPH_ID_4_SPEC {
    const RESET_VALUE: Self::Ux = 0x04;
}
