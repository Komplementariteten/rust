#[doc = "Register `KEYR2` reader"]
pub type R = crate::R<KEYR2_SPEC>;
#[doc = "Field `KEYR2` reader - Bank 2 access configuration unlock key"]
pub type KEYR2_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Bank 2 access configuration unlock key"]
    #[inline(always)]
    pub fn keyr2(&self) -> KEYR2_R {
        KEYR2_R::new(self.bits)
    }
}
#[doc = "FLASH key register for bank 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`keyr2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KEYR2_SPEC;
impl crate::RegisterSpec for KEYR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`keyr2::R`](R) reader structure"]
impl crate::Readable for KEYR2_SPEC {}
#[doc = "`reset()` method sets KEYR2 to value 0"]
impl crate::Resettable for KEYR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
