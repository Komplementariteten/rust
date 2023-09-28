#[doc = "Register `CPUPR3` reader"]
pub type R = crate::R<CPUPR3_SPEC>;
#[doc = "Field `PR82` reader - Configurable event inputs x+64 Pending bit"]
pub type PR82_R = crate::BitReader;
#[doc = "Field `PR84` reader - Configurable event inputs x+64 Pending bit"]
pub type PR84_R = crate::BitReader;
#[doc = "Field `PR85` reader - Configurable event inputs x+64 Pending bit"]
pub type PR85_R = crate::BitReader;
#[doc = "Field `PR86` reader - Configurable event inputs x+64 Pending bit"]
pub type PR86_R = crate::BitReader;
impl R {
    #[doc = "Bit 18 - Configurable event inputs x+64 Pending bit"]
    #[inline(always)]
    pub fn pr82(&self) -> PR82_R {
        PR82_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - Configurable event inputs x+64 Pending bit"]
    #[inline(always)]
    pub fn pr84(&self) -> PR84_R {
        PR84_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Configurable event inputs x+64 Pending bit"]
    #[inline(always)]
    pub fn pr85(&self) -> PR85_R {
        PR85_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Configurable event inputs x+64 Pending bit"]
    #[inline(always)]
    pub fn pr86(&self) -> PR86_R {
        PR86_R::new(((self.bits >> 22) & 1) != 0)
    }
}
#[doc = "EXTI pending register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpupr3::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CPUPR3_SPEC;
impl crate::RegisterSpec for CPUPR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpupr3::R`](R) reader structure"]
impl crate::Readable for CPUPR3_SPEC {}
#[doc = "`reset()` method sets CPUPR3 to value 0"]
impl crate::Resettable for CPUPR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
