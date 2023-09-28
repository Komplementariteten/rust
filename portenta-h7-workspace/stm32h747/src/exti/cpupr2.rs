#[doc = "Register `CPUPR2` reader"]
pub type R = crate::R<CPUPR2_SPEC>;
#[doc = "Field `PR49` reader - Configurable event inputs x+32 Pending bit"]
pub type PR49_R = crate::BitReader;
#[doc = "Field `PR51` reader - Configurable event inputs x+32 Pending bit"]
pub type PR51_R = crate::BitReader;
impl R {
    #[doc = "Bit 17 - Configurable event inputs x+32 Pending bit"]
    #[inline(always)]
    pub fn pr49(&self) -> PR49_R {
        PR49_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19 - Configurable event inputs x+32 Pending bit"]
    #[inline(always)]
    pub fn pr51(&self) -> PR51_R {
        PR51_R::new(((self.bits >> 19) & 1) != 0)
    }
}
#[doc = "EXTI pending register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpupr2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CPUPR2_SPEC;
impl crate::RegisterSpec for CPUPR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpupr2::R`](R) reader structure"]
impl crate::Readable for CPUPR2_SPEC {}
#[doc = "`reset()` method sets CPUPR2 to value 0"]
impl crate::Resettable for CPUPR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
