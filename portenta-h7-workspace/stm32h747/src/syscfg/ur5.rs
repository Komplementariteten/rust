#[doc = "Register `UR5` reader"]
pub type R = crate::R<UR5_SPEC>;
#[doc = "Field `MESAD_1` reader - Mass erase secured area disabled for bank 1"]
pub type MESAD_1_R = crate::BitReader;
#[doc = "Field `WRPS_1` reader - Write protection for flash bank 1"]
pub type WRPS_1_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Mass erase secured area disabled for bank 1"]
    #[inline(always)]
    pub fn mesad_1(&self) -> MESAD_1_R {
        MESAD_1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 16:23 - Write protection for flash bank 1"]
    #[inline(always)]
    pub fn wrps_1(&self) -> WRPS_1_R {
        WRPS_1_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
#[doc = "SYSCFG user register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ur5::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UR5_SPEC;
impl crate::RegisterSpec for UR5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ur5::R`](R) reader structure"]
impl crate::Readable for UR5_SPEC {}
#[doc = "`reset()` method sets UR5 to value 0"]
impl crate::Resettable for UR5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
