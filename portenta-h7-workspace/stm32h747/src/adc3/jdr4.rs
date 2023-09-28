#[doc = "Register `JDR4` reader"]
pub type R = crate::R<JDR4_SPEC>;
#[doc = "Field `JDATA4` reader - ADC group injected sequencer rank 4 conversion data"]
pub type JDATA4_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - ADC group injected sequencer rank 4 conversion data"]
    #[inline(always)]
    pub fn jdata4(&self) -> JDATA4_R {
        JDATA4_R::new(self.bits)
    }
}
#[doc = "ADC group injected sequencer rank 4 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`jdr4::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct JDR4_SPEC;
impl crate::RegisterSpec for JDR4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`jdr4::R`](R) reader structure"]
impl crate::Readable for JDR4_SPEC {}
#[doc = "`reset()` method sets JDR4 to value 0"]
impl crate::Resettable for JDR4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
