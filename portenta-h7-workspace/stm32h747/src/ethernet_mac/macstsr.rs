#[doc = "Register `MACSTSR` reader"]
pub type R = crate::R<MACSTSR_SPEC>;
#[doc = "Field `TSS` reader - TSS"]
pub type TSS_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - TSS"]
    #[inline(always)]
    pub fn tss(&self) -> TSS_R {
        TSS_R::new(self.bits)
    }
}
#[doc = "System time seconds register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macstsr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACSTSR_SPEC;
impl crate::RegisterSpec for MACSTSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macstsr::R`](R) reader structure"]
impl crate::Readable for MACSTSR_SPEC {}
#[doc = "`reset()` method sets MACSTSR to value 0"]
impl crate::Resettable for MACSTSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
