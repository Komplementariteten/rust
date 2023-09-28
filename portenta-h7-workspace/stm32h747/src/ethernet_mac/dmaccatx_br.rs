#[doc = "Register `DMACCATxBR` reader"]
pub type R = crate::R<DMACCATX_BR_SPEC>;
#[doc = "Field `CURTBUFAPTR` reader - Application Transmit Buffer Address Pointer"]
pub type CURTBUFAPTR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Application Transmit Buffer Address Pointer"]
    #[inline(always)]
    pub fn curtbufaptr(&self) -> CURTBUFAPTR_R {
        CURTBUFAPTR_R::new(self.bits)
    }
}
#[doc = "Channel current application transmit buffer register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmaccatx_br::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMACCATX_BR_SPEC;
impl crate::RegisterSpec for DMACCATX_BR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmaccatx_br::R`](R) reader structure"]
impl crate::Readable for DMACCATX_BR_SPEC {}
#[doc = "`reset()` method sets DMACCATxBR to value 0"]
impl crate::Resettable for DMACCATX_BR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
