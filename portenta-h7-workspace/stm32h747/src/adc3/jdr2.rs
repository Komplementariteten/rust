#[doc = "Register `JDR2` reader"]
pub type R = crate::R<JDR2_SPEC>;
#[doc = "Field `JDATA2` reader - ADC group injected sequencer rank 2 conversion data"]
pub type JDATA2_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - ADC group injected sequencer rank 2 conversion data"]
    #[inline(always)]
    pub fn jdata2(&self) -> JDATA2_R {
        JDATA2_R::new(self.bits)
    }
}
#[doc = "ADC group injected sequencer rank 2 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`jdr2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct JDR2_SPEC;
impl crate::RegisterSpec for JDR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`jdr2::R`](R) reader structure"]
impl crate::Readable for JDR2_SPEC {}
#[doc = "`reset()` method sets JDR2 to value 0"]
impl crate::Resettable for JDR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
