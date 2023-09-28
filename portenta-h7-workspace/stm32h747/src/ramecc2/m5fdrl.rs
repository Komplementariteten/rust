#[doc = "Register `M5FDRL` reader"]
pub type R = crate::R<M5FDRL_SPEC>;
#[doc = "Field `FDATAL` reader - Failing data low"]
pub type FDATAL_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Failing data low"]
    #[inline(always)]
    pub fn fdatal(&self) -> FDATAL_R {
        FDATAL_R::new(self.bits)
    }
}
#[doc = "RAMECC monitor x failing data low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m5fdrl::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct M5FDRL_SPEC;
impl crate::RegisterSpec for M5FDRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`m5fdrl::R`](R) reader structure"]
impl crate::Readable for M5FDRL_SPEC {}
#[doc = "`reset()` method sets M5FDRL to value 0"]
impl crate::Resettable for M5FDRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
