#[doc = "Register `DMACCATxDR` reader"]
pub type R = crate::R<DMACCATX_DR_SPEC>;
#[doc = "Field `CURTDESAPTR` reader - Application Transmit Descriptor Address Pointer"]
pub type CURTDESAPTR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Application Transmit Descriptor Address Pointer"]
    #[inline(always)]
    pub fn curtdesaptr(&self) -> CURTDESAPTR_R {
        CURTDESAPTR_R::new(self.bits)
    }
}
#[doc = "Channel current application transmit descriptor register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmaccatx_dr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMACCATX_DR_SPEC;
impl crate::RegisterSpec for DMACCATX_DR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmaccatx_dr::R`](R) reader structure"]
impl crate::Readable for DMACCATX_DR_SPEC {}
#[doc = "`reset()` method sets DMACCATxDR to value 0"]
impl crate::Resettable for DMACCATX_DR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
