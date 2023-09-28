#[doc = "Register `MACATSSR` reader"]
pub type R = crate::R<MACATSSR_SPEC>;
#[doc = "Field `AUXTSHI` reader - AUXTSHI"]
pub type AUXTSHI_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - AUXTSHI"]
    #[inline(always)]
    pub fn auxtshi(&self) -> AUXTSHI_R {
        AUXTSHI_R::new(self.bits)
    }
}
#[doc = "Auxiliary timestamp seconds register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macatssr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACATSSR_SPEC;
impl crate::RegisterSpec for MACATSSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macatssr::R`](R) reader structure"]
impl crate::Readable for MACATSSR_SPEC {}
#[doc = "`reset()` method sets MACATSSR to value 0"]
impl crate::Resettable for MACATSSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
