#[doc = "Register `AXI_PERIPH_ID_3` reader"]
pub type R = crate::R<AXI_PERIPH_ID_3_SPEC>;
#[doc = "Field `CUST_MOD_NUM` reader - Customer modification"]
pub type CUST_MOD_NUM_R = crate::FieldReader;
#[doc = "Field `REV_AND` reader - Customer version"]
pub type REV_AND_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Customer modification"]
    #[inline(always)]
    pub fn cust_mod_num(&self) -> CUST_MOD_NUM_R {
        CUST_MOD_NUM_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Customer version"]
    #[inline(always)]
    pub fn rev_and(&self) -> REV_AND_R {
        REV_AND_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "AXI interconnect - peripheral ID3 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`axi_periph_id_3::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AXI_PERIPH_ID_3_SPEC;
impl crate::RegisterSpec for AXI_PERIPH_ID_3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`axi_periph_id_3::R`](R) reader structure"]
impl crate::Readable for AXI_PERIPH_ID_3_SPEC {}
#[doc = "`reset()` method sets AXI_PERIPH_ID_3 to value 0x04"]
impl crate::Resettable for AXI_PERIPH_ID_3_SPEC {
    const RESET_VALUE: Self::Ux = 0x04;
}
