#[doc = "Register `UR8` reader"]
pub type R = crate::R<UR8_SPEC>;
#[doc = "Field `MEPAD_2` reader - Mass erase protected area disabled for bank 2"]
pub type MEPAD_2_R = crate::BitReader;
#[doc = "Field `MESAD_2` reader - Mass erase secured area disabled for bank 2"]
pub type MESAD_2_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Mass erase protected area disabled for bank 2"]
    #[inline(always)]
    pub fn mepad_2(&self) -> MEPAD_2_R {
        MEPAD_2_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 16 - Mass erase secured area disabled for bank 2"]
    #[inline(always)]
    pub fn mesad_2(&self) -> MESAD_2_R {
        MESAD_2_R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[doc = "SYSCFG user register 8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ur8::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UR8_SPEC;
impl crate::RegisterSpec for UR8_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ur8::R`](R) reader structure"]
impl crate::Readable for UR8_SPEC {}
#[doc = "`reset()` method sets UR8 to value 0"]
impl crate::Resettable for UR8_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
