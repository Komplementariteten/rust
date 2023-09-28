#[doc = "Register `CDR2` reader"]
pub type R = crate::R<CDR2_SPEC>;
#[doc = "Field `RDATA_ALT` reader - Regular data of the master/slave alternated ADCs"]
pub type RDATA_ALT_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Regular data of the master/slave alternated ADCs"]
    #[inline(always)]
    pub fn rdata_alt(&self) -> RDATA_ALT_R {
        RDATA_ALT_R::new(self.bits)
    }
}
#[doc = "ADC x common regular data register for 32-bit dual mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cdr2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CDR2_SPEC;
impl crate::RegisterSpec for CDR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cdr2::R`](R) reader structure"]
impl crate::Readable for CDR2_SPEC {}
#[doc = "`reset()` method sets CDR2 to value 0"]
impl crate::Resettable for CDR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
